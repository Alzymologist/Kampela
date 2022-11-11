use parity_scale_codec::{Decode, Encode};
use std::sync::Arc;

use crate::storage::SpecsKey;

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct DerivationInfo {
    pub chains: Vec<Vec<u8>>,
    pub cut_path: String,
    pub has_pwd: bool,
}

impl DerivationInfo {
    pub fn new(specs_key_set: Vec<Arc<SpecsKey>>, cut_path: String, has_pwd: bool) -> Self {
        Self {
            chains: specs_key_set.iter().map(|a| a.as_db_key()).collect(),
            cut_path,
            has_pwd,
        }
    }
}
