use actor_utils::*;
use bls_signatures::Serialize;
use criterion::{criterion_group, criterion_main, Criterion};
use fvm_integration_tests::tester::Account;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::address::Address;
use fvm_shared::crypto::signature::Signature;
use quarry_single_verify::VerifyParams;
use quarry_single_verify::WASM_BINARY as SINGLE_VERIFY_BIN;

macro_rules! bench_single_verify {
    ($name:ident, $num:expr) => {
        fn $name(c: &mut Criterion) {
            setup_logs();

            // The number of signatures in aggregate
            const NUM_SIGS: usize = 1;
            let mut tester = new_tester();

            let sender: [Account; 1] = tester.create_accounts().unwrap();

            let (actor_address, mut executor) = setup_actor(tester, SINGLE_VERIFY_BIN);

            let mut i = 1;

            c.bench_function(&format!("single_verify {}", $num), |b| {
                let (aggregated_signature, _, public_keys, data) =
                    bls_utils::make_sig_safe(NUM_SIGS, $num);
                let public_key: &[u8] = &public_keys[0].as_bytes();
                let addr: Address = match Address::new_bls(public_key) {
                    Ok(a) => a,
                    Err(err) => {
                        panic!("failed to generate an address from bls sig: {:?}", err);
                    }
                };
                let signature: Signature = Signature::new_bls(aggregated_signature.as_bytes());

                let params = VerifyParams {
                    signature: signature,
                    address: addr,
                    msg: data[0].clone(),
                };

                let raw_bytes = match RawBytes::serialize(params) {
                    Ok(b) => b,
                    Err(err) => {
                        panic!("failed to serialize params {:?}", err);
                    }
                };

                b.iter(|| {
                    call_function(
                        &mut executor,
                        actor_address,
                        raw_bytes.clone(),
                        sender,
                        &mut i,
                    );
                })
            });
        }
    };
}

bench_single_verify!(bench_verify_64, 64);
bench_single_verify!(bench_verify_128, 128);
bench_single_verify!(bench_verify_256, 256);
bench_single_verify!(bench_verify_512, 512);
bench_single_verify!(bench_verify_1024, 1024);

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets =
        bench_verify_64,
        bench_verify_128,
        bench_verify_256,
        bench_verify_512,
        bench_verify_1024
}

criterion_main!(benches);
