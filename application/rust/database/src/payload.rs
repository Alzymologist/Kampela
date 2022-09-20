use frame_metadata::RuntimeMetadataV14;
use sp_core::H256;
use sp_runtime::{MultiSignature, MultiSigner};
use std::convert::TryInto;

use crate::error::Error;
use crate::storage::Encryption;
use crate::utils::read_meta;

pub struct Transaction {
    pub genesis_hash: H256,
    pub meta_v14: RuntimeMetadataV14,
    pub meta_signature: MultiSignature,
    pub signable_transaction: Vec<u8>,
    pub signer: MultiSigner,
}

impl Transaction {
    pub fn new(db_path: &str, mut payload: &[u8]) -> Result<Self, Error> {
        let encryption = match payload.get(..3) {
            Some(prelude) => {
                payload = &payload[3..];
                if prelude[0] != 0x53 {
                    return Err(Error::NotSubstrate);
                }
                if prelude[2] != 0x00 && prelude[2] != 0x02 {
                    return Err(Error::NotTransactionQr);
                }
                Encryption::from_symbol(prelude[1])?
            }
            None => return Err(Error::TooShort),
        };
        let signer = match payload.get(0..encryption.key_length()) {
            Some(public_key_slice) => {
                payload = &payload[encryption.key_length()..];
                match encryption {
                    Encryption::Ed25519 => MultiSigner::Ed25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Sr25519 => MultiSigner::Sr25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Ecdsa => MultiSigner::Ecdsa(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                }
            }
            None => return Err(Error::TooShort),
        };
        if payload.len() >= H256::len_bytes() {
            let genesis_hash = H256(
                payload[payload.len() - H256::len_bytes()..]
                    .try_into()
                    .expect("stable known length"),
            );
            let metadata_value = read_meta(db_path, genesis_hash)?;
            let signable_transaction = payload[..payload.len() - H256::len_bytes()].to_vec();
            Ok(Transaction {
                genesis_hash,
                meta_v14: metadata_value.metadata(),
                meta_signature: metadata_value.signature(),
                signable_transaction,
                signer,
            })
        } else {
            Err(Error::TooShort)
        }
    }
}
