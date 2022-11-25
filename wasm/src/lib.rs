use bls12_381::G2Projective;
use bls_signatures::*;
use rand::Rng;

pub fn make_sig(num: usize) -> (Signature, Vec<G2Projective>, Vec<PublicKey>) {
    let rng = &mut rand::thread_rng();
    // generate private keys
    let private_keys: Vec<_> = (0..num).map(|_| PrivateKey::generate(rng)).collect();

    // generate messages
    let messages: Vec<Vec<u8>> = (0..num)
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

    (aggregated_signature, hashes, public_keys)
}

macro_rules! verify_simd {
    ($name:ident, $stuff:expr) => {
        pub fn $name() {
            verify(&$stuff.0, &$stuff.1, &$stuff.2);
        }
    };
}

verify_simd!(verify_1, make_sig(1));
verify_simd!(verify_10, make_sig(10));
verify_simd!(verify_100, make_sig(100));
verify_simd!(verify_1000, make_sig(1000));
