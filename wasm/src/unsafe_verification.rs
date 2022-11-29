use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
    Bls12, G1Affine, G2Affine, G1Projective, G2Projective, Gt, MillerLoopResult
};
#[cfg(not(feature = "actor"))]
use bls12_381::{Scalar, hash_to_curve::HashToField};
use pairing_lib::{MultiMillerLoop, group::Curve};
use thiserror::Error;
#[cfg(not(feature = "actor"))]
use rand::Rng;
#[cfg(not(feature = "actor"))]
use rand_core::{CryptoRng, RngCore};
#[cfg(not(feature = "actor"))]
use hkdf::Hkdf;
#[cfg(not(feature = "actor"))]
use sha2::{Sha256,  digest::generic_array::GenericArray, digest::generic_array::typenum::U48};

const CSUITE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
const G2_COMPRESSED_SIZE: usize = 96;
const G1_COMPRESSED_SIZE: usize = 48;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Size mismatch")]
    SizeMismatch,
    #[error("Group decode error")]
    GroupDecode,
    #[error("Empty input")]
    ZeroSizedInput
}

#[cfg(not(feature = "actor"))]
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

#[cfg(not(feature = "actor"))]
pub fn public_key(private: &Scalar) -> G1Projective {
    let mut pk = G1Projective::generator();
    pk *= private;
    pk
}
#[cfg(not(feature = "actor"))]
pub fn sign<T: AsRef<[u8]>>(private: &Scalar, message: T) -> G2Affine {
    let mut p = hash(message.as_ref());
    p *= private;

    p.into()
}

#[cfg(not(feature = "actor"))]
pub fn aggregate(signatures: &[G2Affine]) -> Result<G2Affine, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
    }

    let res = signatures
        .into_iter()
        .fold(G2Projective::identity(), |acc, signature| {
            acc + signature
        });

    Ok(res.into())
}

#[cfg(not(feature = "actor"))]
pub fn make_sig_unsafe(num: usize) -> (G2Affine, Vec<u8>, Vec<G1Projective>) {
    let rng = &mut rand::thread_rng();
    // generate private keys
    let private_keys: Vec<_> = (0..num).map(|_| generate_pk(rng)).collect();

    // generate messages
    let message: Vec<u8> =  (0..num*64).map(|_| rng.gen()).collect();
    // sign messages
    let sigs = private_keys
        .iter()
        .map(|pk| sign(pk, message.clone()))
        .collect::<Vec<G2Affine>>();

    let aggregated_signature = aggregate(&sigs).unwrap();

    let public_keys = private_keys
        .iter()
        .map(|pk| public_key(pk))
        .collect::<Vec<_>>();

    (aggregated_signature, message, public_keys)
}

pub fn g2_from_slice(raw: &[u8]) -> Result<G2Affine, Error> {
    if raw.len() != G2_COMPRESSED_SIZE {
        return Err(Error::SizeMismatch);
    }

    let mut res = [0u8; G2_COMPRESSED_SIZE];
    res.copy_from_slice(raw);

    Option::from(G2Affine::from_compressed(&res)).ok_or(Error::GroupDecode)
}


pub fn g1_from_slice(raw: &[u8]) -> Result<G1Projective, Error> {
    if raw.len() != G1_COMPRESSED_SIZE {
        return Err(Error::SizeMismatch);
    }

    let mut res = [0u8; G1_COMPRESSED_SIZE];
    res.as_mut().copy_from_slice(raw);
    let affine: G1Affine =
        Option::from(G1Affine::from_compressed(&res)).ok_or(Error::GroupDecode)?;

    Ok(affine.into())
}


/// Hash the given message, as used in the signature.
pub fn hash(msg: &[u8]) -> G2Projective {
    <G2Projective as HashToCurve<ExpandMsgXmd<sha2::Sha256>>>::hash_to_curve(msg, CSUITE)
}

// Note that this does not enforce that messages are distinct as a countermeasure against BLS's rogue-key attack.
// See Section 3.1. of the IRTF's BLS signatures spec: https://tools.ietf.org/html/draft-irtf-cfrg-bls-signature-02#section-3.1
// as such we need to find other methods to counter rogue-key attacks (eg. public key registration on the contract)
pub fn aggregate_bls_verify(
    signature: &G2Affine,
    message: &[u8],
    public_keys: &[G1Projective],
) -> bool {
    let hash = hash(message);

    // zero key & single hash should fail
    if public_keys[0].is_identity().into() {
        return false;
    }

    let mut is_valid = true;

    let mut ml = public_keys
        .iter()
        .map(|pk| {
            if pk.is_identity().into() {
                is_valid = false;
            }
            let pk = pk.to_affine();
            let h = G2Affine::from(hash).into();
            Bls12::multi_miller_loop(&[(&pk, &h)])
        })
        .fold(MillerLoopResult::default(), |acc, cur| acc + cur);

    if !is_valid {
        return false;
    }

    let g1_neg = -G1Affine::generator();

    ml += Bls12::multi_miller_loop(&[(&g1_neg, &(*signature).into())]);

    ml.final_exponentiation() == Gt::identity()
}




