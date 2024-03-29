use bindings::{fp_export_impl, VerifyParams};
use bls12_381::{hash_to_curve::HashToField, Scalar};
use bls12_381::{G1Projective, G2Affine, G2Projective};
use bls_signatures::{verify, PublicKey, Serialize as BlsSer, Signature};
use bls_wasm_unsafe::Error;
use bls_wasm_unsafe::{g1_from_slice, g2_from_slice};
use hkdf::Hkdf;
use rand::Rng;
use rand_core::{CryptoRng, RngCore};
use sha2::{digest::generic_array::typenum::U48, digest::generic_array::GenericArray, Sha256};

/// Generate a new private key.
pub fn generate_pk<R: RngCore + CryptoRng>(rng: &mut R) -> Scalar {
    // IKM must be at least 32 bytes long:
    // https://tools.ietf.org/html/draft-irtf-cfrg-bls-signature-00#section-2.3
    let mut data = [0u8; 32];
    rng.try_fill_bytes(&mut data)
        .expect("unable to produce secure randomness");

    // "BLS-SIG-KEYGEN-SALT-"
    const SALT: &[u8] = b"BLS-SIG-KEYGEN-SALT-";

    let data = data.as_ref();
    // HKDF-Extract
    let mut msg = data.as_ref().to_vec();
    // append zero byte
    msg.push(0);
    let prk = Hkdf::<Sha256>::new(Some(SALT), &msg);

    // HKDF-Expand
    // `result` has enough length to hold the output from HKDF expansion
    let mut result = GenericArray::<u8, U48>::default();
    assert!(prk.expand(&[0, 48], &mut result).is_ok());

    Scalar::from_okm(&result)
}

pub fn public_key(private: &Scalar) -> G1Projective {
    let mut pk = G1Projective::generator();
    pk *= private;
    pk
}

pub fn sign_unsafe<T: AsRef<[u8]>>(private: &Scalar, message: T) -> G2Affine {
    let mut p = bls_wasm_unsafe::hash(message.as_ref());
    p *= private;

    p.into()
}

pub fn aggregate_unsafe(signatures: &[G2Affine]) -> Result<G2Affine, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
    }

    let res = signatures
        .iter()
        .fold(G2Projective::identity(), |acc, signature| acc + signature);

    Ok(res.into())
}

pub fn make_sig_unsafe(num: usize, msg_len: usize) -> (G2Affine, Vec<u8>, Vec<G1Projective>) {
    let rng = &mut rand::thread_rng();
    // generate private keys
    let private_keys: Vec<_> = (0..num).map(|_| generate_pk(rng)).collect();

    // generate messages
    let message: Vec<u8> = (0..num * msg_len).map(|_| rng.gen()).collect();
    // sign messages
    let sigs = private_keys
        .iter()
        .map(|pk| sign_unsafe(pk, message.clone()))
        .collect::<Vec<G2Affine>>();

    let aggregated_signature = aggregate_unsafe(&sigs).unwrap();

    let public_keys = private_keys
        .iter()
        .map(|pk| public_key(pk))
        .collect::<Vec<_>>();

    (aggregated_signature, message, public_keys)
}

pub fn make_sig_safe(
    num: usize,
    msg_len: usize,
) -> (
    bls_signatures::Signature,
    Vec<G2Projective>,
    Vec<bls_signatures::PublicKey>,
    Vec<Vec<u8>>,
) {
    let rng = &mut rand::thread_rng();
    // generate private keys
    let private_keys: Vec<_> = (0..num)
        .map(|_| bls_signatures::PrivateKey::generate(rng))
        .collect();

    // generate messages
    let messages: Vec<Vec<u8>> = (0..num)
        .map(|_| (0..msg_len).map(|_| rng.gen()).collect())
        .collect();

    // sign messages
    let sigs = messages
        .iter()
        .zip(&private_keys)
        .map(|(message, pk)| pk.sign(message))
        .collect::<Vec<bls_signatures::Signature>>();

    let aggregated_signature = bls_signatures::aggregate(&sigs).unwrap();

    let hashes = messages
        .iter()
        .map(|message| bls_signatures::hash(message))
        .collect::<Vec<_>>();
    let public_keys = private_keys
        .iter()
        .map(|pk| pk.public_key())
        .collect::<Vec<_>>();

    assert!(bls_signatures::verify(
        &aggregated_signature,
        &hashes,
        &public_keys
    ));

    (aggregated_signature, hashes, public_keys, messages)
}

#[fp_export_impl(bindings)]
fn run_sig_verification(params: VerifyParams) -> bool {
    let sig_bytes = params.aggregate_signature;
    let hash_bytes = params.hashes;
    let pub_keys_bytes = params.pub_keys;

    let aggregated_signature = Signature::from_bytes(&sig_bytes).unwrap();

    let hashes: Vec<G2Projective> = hash_bytes
        .iter()
        .map(|hash_val| g2_from_slice(hash_val).unwrap())
        .map(|hash_val| G2Projective::from(hash_val))
        .collect();

    let public_keys: Vec<PublicKey> = pub_keys_bytes
        .iter()
        .map(|key| g1_from_slice(key).unwrap())
        .map(|key| PublicKey::from(key))
        .collect();

    verify(&aggregated_signature, &hashes, &public_keys)
}
