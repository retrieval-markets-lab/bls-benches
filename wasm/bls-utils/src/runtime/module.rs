#[path = "../spec/mod.rs"]
mod spec;

use bls_signatures::Serialize;
use bls_utils::make_sig_safe;
use group::GroupEncoding;
use spec::{bindings::Runtime, types::VerifyParams};
use std::{fs::read, path::Path};

pub fn run_wasm_module(path: &Path) {
    let bytes = read(path).unwrap();
    let mut runtime = Runtime::new(bytes).unwrap();

    let (aggregated_signature, hashes, public_keys, _) = make_sig_safe(1, 64);

    let signature_bytes = aggregated_signature.as_bytes().to_vec();
    let public_keys: Vec<_> = public_keys.iter().map(|x| x.as_bytes()).collect();
    let hash_vec: Vec<_> = hashes
        .iter()
        .map(|x| x.to_bytes().as_mut().to_vec())
        .collect();

    let params = VerifyParams {
        aggregate_signature: signature_bytes,
        pub_keys: public_keys,
        hashes: hash_vec,
    };

    let res = runtime.run_sig_verification(params);
    println!("{:?}", res)
}
