use bls_signatures::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

macro_rules! bench_verify {
    ($name:ident, $num:expr) => {
        fn $name(c: &mut Criterion) {
            let rng = &mut rand::thread_rng();
            // generate private keys
            let private_keys: Vec<_> = (0..$num).map(|_| PrivateKey::generate(rng)).collect();

            // generate messages
            let messages: Vec<Vec<u8>> = (0..$num)
                .map(|_| (0..64).map(|_| rng.gen()).collect())
                .collect();

            // sign messages
            let sigs = messages
                .iter()
                .zip(&private_keys)
                .map(|(message, pk)| pk.sign(message))
                .collect::<Vec<Signature>>();

            let aggregated_signature = aggregate(&sigs).unwrap();

            let hashes = messages
                .iter()
                .map(|message| hash(message))
                .collect::<Vec<_>>();
            let public_keys = private_keys
                .iter()
                .map(|pk| pk.public_key())
                .collect::<Vec<_>>();
            c.bench_function(&format!("verify {}", $num), |b| {
                b.iter(|| verify(&aggregated_signature, &hashes, &public_keys))
            });
        }
    };
}

bench_verify!(bench_verify_1, 1);
bench_verify!(bench_verify_10, 10);
bench_verify!(bench_verify_100, 100);
bench_verify!(bench_verify_1000, 1000);

criterion_group!(
    benches,
    bench_verify_1,
    bench_verify_10,
    bench_verify_100,
    bench_verify_1000
);

criterion_main!(benches);
