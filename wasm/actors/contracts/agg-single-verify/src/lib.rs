#[cfg(not(target_arch = "wasm32"))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod blockstore;
use crate::blockstore::Blockstore;
use bls_wasm_unsafe::{aggregate_bls_verify, g1_from_slice, g2_from_slice};
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::message::params_raw;
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_shared::crypto::signature::Signature;
use fvm_shared::ActorID;

/// A macro to abort concisely.
macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };

}

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64,
}

/// The params object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct VerifyParams {
    pub aggregate_signature: Signature,
    pub pub_keys: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl State {
    pub fn load() -> Self {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}

/// The actor's WASM entrypoint. It takes the ID of the parameters block,
/// and returns the ID of the return value block, or NO_DATA_BLOCK_ID if no
/// return value.
#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    // Conduct method dispatch. Handle input parameters and return data.
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => constructor(),
        2 => {
            let params = params_raw(params).unwrap().1;
            let params = match RawBytes::new(params).deserialize() {
                Ok(p) => p,
                Err(err) => {
                    abort!(USR_SERIALIZATION, "failed to deserialize params: {}", err);
                }
            };
            verify_sig(params)
        }
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    // Insert the return data block if necessary, and return the correct
    // block ID.
    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

/// The constructor populates the initial state.
///
/// Method num 1. This is part of the Filecoin calling convention.
/// InitActor#Exec will call the constructor on method_num = 1.
pub fn constructor() -> Option<RawBytes> {
    // This constant should be part of the SDK.
    const INIT_ACTOR_ADDR: ActorID = 1;

    if sdk::message::caller() != INIT_ACTOR_ADDR {
        abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
    }

    let state = State::default();
    state.save();
    None
}

/// Method num 2.
pub fn verify_sig(params: VerifyParams) -> Option<RawBytes> {
    let mut state = State::load();
    state.count += 1;
    state.save();

    if params.data.is_empty() {
        abort!(USR_ILLEGAL_STATE, "data field is empty");
    }

    let sig = match g2_from_slice(params.aggregate_signature.bytes()) {
        Ok(v) => v,
        Err(err) => {
            abort!(USR_ILLEGAL_STATE, "invalid signature {:?}", err);
        }
    };

    let pk_map_results: Result<Vec<_>, _> =
        params.pub_keys.iter().map(|x| g1_from_slice(x)).collect();

    let pks = match pk_map_results {
        Ok(v) => v,
        Err(err) => {
            abort!(USR_ILLEGAL_STATE, "invalid pub key {:?}", err);
        }
    };
    // Does the aggregate verification

    let verification = aggregate_bls_verify(&sig, &params.data, &pks[..]);

    let ret = to_vec(format!("Call #{} ended with {:?}!", &state.count, verification).as_str());
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
