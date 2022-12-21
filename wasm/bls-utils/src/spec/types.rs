#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VerifyParams {
    pub aggregate_signature: Vec<u8>,
    pub pub_keys: Vec<Vec<u8>>,
    pub hashes: Vec<Vec<u8>>,
}
