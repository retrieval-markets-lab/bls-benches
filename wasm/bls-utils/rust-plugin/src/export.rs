use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn run_sig_verification(data: VerifyParams) -> bool;
