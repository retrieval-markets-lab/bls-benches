use actor_utils::*;
use bls_signatures::Serialize;
use criterion::{criterion_group, criterion_main, Criterion};
use fvm_integration_tests::tester::Account;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::crypto::signature::Signature;
use quarry_multi_verify::VerifyParams as MultiVerifyParams;
use quarry_multi_verify::WASM_BINARY as MULTI_VERIFY_BIN;

macro_rules! bench_verify {
    ($name:ident, $num:expr) => {
        fn $name(c: &mut Criterion) {
            setup_logs();

            const MESSAGE_LEN: usize = 64;

            let mut tester = new_tester();

            let sender: [Account; 1] = tester.create_accounts().unwrap();

            let (actor_address, mut executor) = setup_actor(tester, MULTI_VERIFY_BIN);

            let mut i = 1;

            c.bench_function(&format!("agg_verify_safe {}", $num), |b| {
                let (aggregated_signature, _, public_keys, data) =
                    bls_utils::make_sig_safe($num, MESSAGE_LEN);
                let signature: Signature = Signature::new_bls(aggregated_signature.as_bytes());
                let public_keys: Vec<_> = public_keys.iter().map(|x| x.as_bytes()).collect();

                let params = MultiVerifyParams {
                    aggregate_signature: signature,
                    pub_keys: public_keys,
                    data: data.iter().map(|d| d.to_vec()).collect(),
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

bench_verify!(bench_verify_64, 64);
bench_verify!(bench_verify_128, 128);
bench_verify!(bench_verify_256, 256);
bench_verify!(bench_verify_512, 512);
bench_verify!(bench_verify_1024, 1024);

criterion_group!(
    benches,
    bench_verify_64,
    bench_verify_128,
    bench_verify_256,
    bench_verify_512,
    bench_verify_1024
);

criterion_main!(benches);
