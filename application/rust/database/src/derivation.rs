use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct DerivationInfo {
    pub chains: Vec<Vec<u8>>,
    pub cut_path: String,
    pub has_pwd: bool,
}

impl DerivationInfo {
    pub fn new(chains: Vec<Vec<u8>>, cut_path: String, has_pwd: bool) -> Self {
        Self {
            chains,
            cut_path,
            has_pwd,
        }
    }
}
