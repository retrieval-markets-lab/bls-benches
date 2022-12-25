#[path = "../src/spec/mod.rs"] mod spec;

use spec::{bindings::Runtime, types::VerifyParams};
use std::{fs::read, path::Path};
use bls_signatures::Serialize;
use group::GroupEncoding;
use bls_utils::make_sig_safe;
use criterion::{criterion_group, criterion_main, Criterion};



macro_rules! bench_verify {
    ($name:ident, $num:expr) => {

        fn $name(c: &mut Criterion) {
            c.bench_function(&format!("wasm_verify {}", $num), |b| {
                let wasm_file = Path::new("./src/wasm-files/bls_utils.wasm");
                let bytes = read(wasm_file).unwrap();
                let mut runtime = Runtime::new(&bytes).unwrap();
            
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

                b.iter(|| runtime.run_sig_verification(params.clone()))
            });
        }
    };
}

bench_verify!(bench_wasm_verify_1, 1);
bench_verify!(bench_wasm_verify_10, 10);
bench_verify!(bench_wasm_verify_100, 100);
bench_verify!(bench_wasm_verify_512, 512);
bench_verify!(bench_wasm_verify_1000, 1000);

criterion_group!(
    wasm_benches,
    bench_wasm_verify_1,
    bench_wasm_verify_10,
    bench_wasm_verify_100,
    bench_wasm_verify_512,
    bench_wasm_verify_1000
);

criterion_main!(wasm_benches);
