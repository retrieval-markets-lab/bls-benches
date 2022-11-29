pub mod unsafe_verification;

#[cfg(not(feature = "actor"))]
use bls12_381::G2Projective;
#[cfg(not(feature = "actor"))]
use bls_signatures::*;
#[cfg(not(feature = "actor"))]
use rand::Rng;

#[cfg(not(feature = "actor"))]
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
