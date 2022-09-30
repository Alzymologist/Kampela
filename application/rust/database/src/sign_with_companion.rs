use parity_scale_codec::{Decode, Encode};

pub trait SignByCompanion: Send + Sync + std::fmt::Debug {
    fn make_signature(&self, data: Vec<u8>) -> Vec<u8>;
}

pub struct SignatureMaker {
    pub signature_maker: Box<dyn SignByCompanion>,
}

#[derive(Debug, Decode, Encode)]
pub struct TransferData {
    encoded_data: Vec<u8>,
    companion_signature: Vec<u8>,
}

impl SignatureMaker {
    pub fn new(signature_maker: Box<dyn SignByCompanion>) -> Self {
        Self { signature_maker }
    }
    pub fn signed_data(&self, encoded_data: Vec<u8>) -> Vec<u8> {
        let companion_signature = self.signature_maker.make_signature(encoded_data.to_owned());
        TransferData {
            encoded_data,
            companion_signature,
        }
        .encode()
    }
}
