use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
    Bls12, G1Affine, G1Projective, G2Affine, G2Projective, Gt, MillerLoopResult,
};
use pairing_lib::{group::Curve, MultiMillerLoop};
use thiserror::Error;

pub const CSUITE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
pub const G2_COMPRESSED_SIZE: usize = 96;
pub const G1_COMPRESSED_SIZE: usize = 48;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Size mismatch")]
    SizeMismatch,
    #[error("Group decode error")]
    GroupDecode,
    #[error("Empty input")]
    ZeroSizedInput,
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
// as such we need to find other methods to counter rogue-key attacks (eg. public key registration on a contract)
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
