use bls_utils::make_sig_unsafe;
use bls_wasm_unsafe::aggregate_bls_verify;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_verify {
    ($name:ident, $num:expr) => {
        fn $name(c: &mut Criterion) {
            let (aggregated_signature, message, public_keys) = make_sig_unsafe($num, 64);
            c.bench_function(&format!("verify_unsafe {}", $num), |b| {
                b.iter(|| aggregate_bls_verify(&aggregated_signature, &message, &public_keys))
            });
        }
    };
}

bench_verify!(bench_verify_unsafe_1, 1);
bench_verify!(bench_verify_unsafe_10, 10);
bench_verify!(bench_verify_unsafe_100, 100);
bench_verify!(bench_verify_unsafe_512, 512);
bench_verify!(bench_verify_unsafe_1000, 1000);

criterion_group!(
    benches_unsafe,
    bench_verify_unsafe_1,
    bench_verify_unsafe_10,
    bench_verify_unsafe_100,
    bench_verify_unsafe_512,
    bench_verify_unsafe_1000
);

criterion_main!(benches_unsafe);
