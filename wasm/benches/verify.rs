use bls_wasm::{make_sig};
use bls_signatures::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_verify {
    ($name:ident, $num:expr) => {
        fn $name(c: &mut Criterion) {
            let (aggregated_signature, hashes, public_keys) = make_sig($num);
            c.bench_function(&format!("verify {}", $num), |b| {
                b.iter(|| verify(&aggregated_signature, &hashes, &public_keys))
            });
        }
    };
}

bench_verify!(bench_verify_1, 1);
bench_verify!(bench_verify_10, 10);
bench_verify!(bench_verify_100, 100);
bench_verify!(bench_verify_512, 512);
bench_verify!(bench_verify_1000, 1000);

criterion_group!(
    benches,
    bench_verify_1,
    bench_verify_10,
    bench_verify_100,
    bench_verify_512,
    bench_verify_1000
);

criterion_main!(benches);
