use bls_signatures::{PrivateKey, Serialize, Signature as BlsSignature};
use fvm::executor::{ApplyKind, Executor};
use fvm_integration_tests::bundle;
use fvm_integration_tests::dummy::DummyExterns;
use fvm_integration_tests::tester::{Account, Tester};
use fvm_ipld_blockstore::MemoryBlockstore;
use fvm_ipld_encoding::tuple::*;
use fvm_shared::address::Address;
use fvm_shared::crypto::signature::ops::{verify_bls_aggregate, verify_bls_sig};
use fvm_shared::crypto::signature::Signature;
use fvm_shared::econ::TokenAmount;
use fvm_shared::message::Message;
use fvm_shared::state::StateTreeVersion;
use fvm_shared::version::NetworkVersion;
use log::info;
use num_traits::Zero;
use quarry_single_verify::WASM_BINARY as SINGLE_VERIFY_BIN;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::sync::Once;
static INIT: Once = Once::new();
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};


/// Setup function that is only run once, even if called multiple times.
fn setup() {
    INIT.call_once(|| {
        colog::init();
    });
}


const WAT: &str = r#"
;; Mock invoke function
(module
  (func (export "invoke") (param $x i32) (result i32)
    (i32.const 1)
  )
)
"#;

#[test]
pub fn dummy_actor() {

    setup();

    #[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
    struct State {
        empty: bool,
    }

    // Instantiate tester
    let bs = MemoryBlockstore::default();
    let bundle_root = bundle::import_bundle(&bs, actors_v10::BUNDLE_CAR).unwrap();
    let mut tester =
        Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap();

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // Get wasm bin
    let wasm_bin = wat::parse_str(WAT).unwrap();

    // Set actor state
    let actor_state = State { empty: true };
    let state_cid = tester.set_state(&actor_state).unwrap();

    // Set actor
    let actor_address = Address::new_id(10000);

    tester
        .set_actor_from_bin(&wasm_bin, state_cid, actor_address, TokenAmount::zero())
        .unwrap();

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();

    // Send message
    let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: 1,
        ..Message::default()
    };

    let res = tester
        .executor
        .unwrap()
        .execute_message(message, ApplyKind::Explicit, 100);

    assert!(res.is_ok());

    info!("Return data {:?}", res.unwrap().msg_receipt.return_data);
}

struct State {
    empty: bool,
}

#[test]
pub fn single_verify() {
    setup();
    // Instantiate tester
    let bs = MemoryBlockstore::default();
    let bundle_root = bundle::import_bundle(&bs, actors_v10::BUNDLE_CAR).unwrap();
    let mut tester =
        Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap();

    #[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
    struct State {
        count: usize,
    }

    // The number of signatures in aggregate
    const NUM_SIGS: usize = 1;
    info!("Using a single signature");

    const MESSAGE_LEN: usize = NUM_SIGS * 64;
    info!("Using a message of length {}", MESSAGE_LEN);

    let rng = &mut ChaCha8Rng::seed_from_u64(11);
    let msg = &mut [0 as u8; MESSAGE_LEN];
    rng.fill_bytes(msg);
    info!("Message [{:?}, {:?} ...]", msg[0], msg[1]);

    let private_key: PrivateKey = PrivateKey::generate(rng);
    let public_key: &[u8] = &private_key.public_key().as_bytes(); 
    
    let signature: Signature =  Signature::new_bls(private_key.sign(msg.clone()).as_bytes());
    info!("Signed message");

    assert!(verify_bls_aggregate(
        &[msg],
        &[&public_key],
        &signature,
    ),);

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // Get wasm bin
    let wasm_bin = SINGLE_VERIFY_BIN.unwrap();
    

    // Set actor state
    let actor_state = State { count: 0 };
    let state_cid = tester.set_state(&actor_state).unwrap();

    // Set actor
    let actor_address = Address::new_id(10000);

    tester
        .set_actor_from_bin(&wasm_bin, state_cid, actor_address, TokenAmount::zero())
        .unwrap();

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();
    let mut executor = tester
    .executor
    .unwrap();

    // Send message
    let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: 1,
        sequence: 0,
        ..Message::default()
    };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100);

    assert!(res.is_ok());

    let raw_bytes =  RawBytes::new(signature.bytes);
     // Send message
     let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: 2,
        sequence: 1,
        params: raw_bytes,
        ..Message::default()
    };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100);


    info!("Return data {:?}", res);
}

// #[ignore]
// #[test]
// pub fn multiple_verifies() {
//     colog::init();
//     // Instantiate tester
//     let bs = MemoryBlockstore::default();
//     let bundle_root = bundle::import_bundle(&bs, actors_v10::BUNDLE_CAR).unwrap();
//     let mut tester =
//         Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap();

//     // The number of signatures in aggregate
//     const NUM_SIGS: usize = 10;
//     info!("Using {} signatures", NUM_SIGS);

//     const MESSAGE_LEN: usize = NUM_SIGS * 64;
//     info!("Using a message of length {}", MESSAGE_LEN);

//     let rng = &mut ChaCha8Rng::seed_from_u64(11);
//     let msg = &mut [0 as u8; MESSAGE_LEN];
//     rng.fill_bytes(msg);
//     info!("Message [{:?}, {:?} ...]", msg[0], msg[1]);

//     let data: Vec<&[u8]> = (0..NUM_SIGS).map(|x| &msg[x * 64..(x + 1) * 64]).collect();

//     let private_keys: Vec<PrivateKey> = (0..NUM_SIGS).map(|_| PrivateKey::generate(rng)).collect();
//     let public_keys: Vec<_> = private_keys
//         .iter()
//         .map(|x| x.public_key().as_bytes())
//         .collect();

//     let signatures: Vec<BlsSignature> = (0..NUM_SIGS)
//         .map(|x| private_keys[x].sign(data[x]))
//         .collect();
//     info!("Signed message");

//     let public_keys_slice: Vec<&[u8]> = public_keys.iter().map(|x| &**x).collect();

//     let calculated_bls_agg =
//         Signature::new_bls(bls_signatures::aggregate(&signatures).unwrap().as_bytes());
//     assert!(verify_bls_aggregate(
//         &data,
//         &public_keys_slice,
//         &calculated_bls_agg
//     ),);
//     info!("Aggregated signatures");

//     let sender: [Account; 1] = tester.create_accounts().unwrap();

//     // Get wasm bin
//     let wasm_bin = wat::parse_str(WAT).unwrap();

//     // Set actor state
//     let actor_state = State { empty: true };
//     let state_cid = tester.set_state(&actor_state).unwrap();

//     // Set actor
//     let actor_address = Address::new_id(10000);

//     tester
//         .set_actor_from_bin(&wasm_bin, state_cid, actor_address, TokenAmount::zero())
//         .unwrap();

//     // Instantiate machine
//     tester.instantiate_machine(DummyExterns).unwrap();

//     // Send message
//     let message = Message {
//         from: sender[0].1,
//         to: actor_address,
//         gas_limit: 1000000000,
//         method_num: 1,
//         ..Message::default()
//     };

//     let res = tester
//         .executor
//         .unwrap()
//         .execute_message(message, ApplyKind::Explicit, 100);

//     assert!(res.is_ok());

//     info!("Return data {:?}", res.unwrap().msg_receipt.return_data);
// }
