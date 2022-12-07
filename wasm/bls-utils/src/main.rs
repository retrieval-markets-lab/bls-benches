 mod lib;
 use fvm_wasm_instrument::{
	gas_metering,
    inject_stack_limiter,
	parity_wasm::{deserialize_buffer, serialize_to_file},
};

use std::{
	fs::read,
	path::Path, io::Read,
};

use lib::{make_sig_safe, run_sig_verification, VerifyParams};
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{RawBytes};
use bls_wasm_unsafe::{aggregate_bls_verify, g2_from_slice, g1_from_slice};

use bls_signatures::{Signature as BlsSignature, Serialize as BlsSerialize, PublicKey, verify};
use bls12_381::{G2Projective, G2Affine};
use fvm_shared::crypto::{signature::Signature};
use group::{GroupEncoding};

const STACK_LIMIT: u32  = 128;

fn inject_fvm_modules() {
    let path:&Path = Path::new("./fixtures/verify.wasm");
    println!("{}", path.display());
    let bytes: Vec<u8> = read(path).unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();    
    let savepath = Path::new("./fixtures").join(format!("{file_name}-fvm-injected.wasm"));
    let module = deserialize_buffer(&bytes).unwrap();

    let mut injected_module  = 
        gas_metering::inject(module, &gas_metering::ConstantCostRules::default(), "env").unwrap();
    injected_module = inject_stack_limiter(injected_module, STACK_LIMIT).unwrap();

    let serialize_result = serialize_to_file(savepath, injected_module);
    let _serialize = match serialize_result {
        Ok(file) => file,
        Err(error) => panic!("Error serializing module to wasm file: {:?}", error),
    };
}

fn main () {

    let (
        aggregated_signature, 
        hashes, 
        public_keys, _
    ) = make_sig_safe(1, 64);
    
    let signature_bytes = Signature::new_bls(aggregated_signature.as_bytes());
    let public_keys: Vec<_> = public_keys.iter().map(|x| x.as_bytes()).collect();
    let hash_vec: Vec<_> = hashes.iter().map(|x| x.to_bytes().as_mut().to_vec()).collect();
   
    let params = VerifyParams {
        aggregate_signature: signature_bytes,
        pub_keys: public_keys,
        hashes: hash_vec
    };

    let params =  RawBytes::serialize(params).unwrap();
    let res = run_sig_verification(params.bytes());

}