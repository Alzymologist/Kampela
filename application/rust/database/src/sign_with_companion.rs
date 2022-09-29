pub trait SignedByCompanion: Send + Sync + std::fmt::Debug {
    fn sign(&self, data: Vec<u8>) -> Vec<u8>;
}

pub struct SignedData {
    pub signed_data: Box<dyn SignedByCompanion>,
}

impl SignedData {
    pub fn new(signed_data: Box<dyn SignedByCompanion>) -> Self {
        Self { signed_data }
    }
    pub fn sign(&self, data: Vec<u8>) -> Vec<u8> {
        let signature = self.signed_data.sign(data.to_owned());
        [data, signature].concat()
    }
}
