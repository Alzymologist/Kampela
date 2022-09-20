use frame_metadata::{RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use sp_core::{ByteArray, H256};
use sp_runtime::MultiSignature;
use std::convert::TryInto;
use substrate_parser::compacts::find_compact;

use crate::error::Error;

pub struct MetadataKey(MetadataKeyContent);

#[derive(Decode, Encode)]
struct MetadataKeyContent {
    genesis_hash: H256,
}

impl MetadataKey {
    pub fn new(genesis_hash: H256) -> Self {
        Self(MetadataKeyContent { genesis_hash })
    }
    pub fn from_db_key(database_key: &IVec) -> Result<Self, Error> {
        Ok(Self(
            MetadataKeyContent::decode(&mut &database_key[..])
                .map_err(|_| Error::DecodeDbMetadataKey)?,
        ))
    }
    pub fn as_db_key(&self) -> Vec<u8> {
        self.0.encode()
    }
    pub fn hash(&self) -> H256 {
        self.0.genesis_hash.to_owned()
    }
}

pub struct MetadataValue(MetadataValueContent);

#[derive(Decode, Encode)]
struct MetadataValueContent {
    metadata: RuntimeMetadataV14,
    signature: MultiSignature,
}

impl MetadataValue {
    pub fn from_db_value(database_value: &IVec) -> Result<Self, Error> {
        Ok(Self(
            MetadataValueContent::decode(&mut &database_value[..])
                .map_err(|_| Error::DecodeDbMetadataValue)?,
        ))
    }
    pub fn as_db_value(&self) -> Vec<u8> {
        self.0.encode()
    }
    pub fn metadata(&self) -> RuntimeMetadataV14 {
        self.0.metadata.to_owned()
    }
    pub fn signature(&self) -> MultiSignature {
        self.0.signature.to_owned()
    }
}

pub struct MetadataStorage {
    pub key: MetadataKey,
    pub value: MetadataValue,
}

pub enum Encryption {
    Ed25519,
    Sr25519,
    Ecdsa,
}

impl Encryption {
    pub fn from_symbol(symbol: u8) -> Result<Self, Error> {
        match symbol {
            0 => Ok(Encryption::Ed25519),
            1 => Ok(Encryption::Sr25519),
            2 => Ok(Encryption::Ecdsa),
            _ => Err(Error::UnknownSigningAlgorithm),
        }
    }
    pub fn key_length(&self) -> usize {
        match &self {
            Encryption::Ed25519 => sp_core::ed25519::Public::LEN,
            Encryption::Sr25519 => sp_core::sr25519::Public::LEN,
            Encryption::Ecdsa => sp_core::ecdsa::Public::LEN,
        }
    }
    pub fn signature_length(&self) -> usize {
        match &self {
            Encryption::Ed25519 => 64,
            Encryption::Sr25519 => 64,
            Encryption::Ecdsa => 65,
        }
    }
}

impl MetadataStorage {
    pub fn from_metadata_qr(mut payload: &[u8]) -> Result<Self, Error> {
        let encryption = match payload.get(..3) {
            Some(prelude) => {
                if prelude[0] != 0x53 {
                    return Err(Error::NotSubstrate);
                }
                if prelude[2] != 0x80 {
                    return Err(Error::NotMetadataQr);
                }
                Encryption::from_symbol(prelude[1])?
            }
            None => return Err(Error::TooShort),
        };
        payload = match payload.get(3 + encryption.key_length()..) {
            Some(a) => a,
            None => return Err(Error::TooShort),
        };
        let length_info =
            find_compact::<u32>(payload).map_err(|_| Error::MetadataQrUnexpectedStructure)?;
        let meta_length = length_info.compact as usize;
        match length_info.start_next_unit {
            Some(start) => match payload.get(..start + meta_length) {
                Some(meta_slice) => {
                    if !meta_slice.starts_with(b"META") {
                        return Err(Error::NoMetaPrefixQr);
                    }
                    let meta_decoded = RuntimeMetadata::decode(&mut &meta_slice[4..])
                        .map_err(|_| Error::MetadataQrDecode)?;
                    let metadata = match meta_decoded {
                        RuntimeMetadata::V14(metadata) => metadata,
                        _ => return Err(Error::OnlyV14SupportedQr),
                    };
                    payload = &payload[start + meta_length..];
                    match payload.get(..H256::len_bytes()) {
                        Some(hash_slice) => {
                            let hash = H256(hash_slice.try_into().expect("stable known length"));
                            payload = &payload[H256::len_bytes()..];
                            match payload.get(..encryption.signature_length()) {
                                Some(signature_slice) => {
                                    let signature = match encryption {
                                        Encryption::Ed25519 => MultiSignature::Ed25519(
                                            signature_slice
                                                .try_into()
                                                .expect("stable known length"),
                                        ),
                                        Encryption::Sr25519 => MultiSignature::Sr25519(
                                            signature_slice
                                                .try_into()
                                                .expect("stable known length"),
                                        ),
                                        Encryption::Ecdsa => MultiSignature::Ecdsa(
                                            signature_slice
                                                .try_into()
                                                .expect("stable known length"),
                                        ),
                                    };
                                    Ok(Self {
                                        key: MetadataKey::new(hash),
                                        value: MetadataValue(MetadataValueContent {
                                            metadata,
                                            signature,
                                        }),
                                    })
                                }
                                None => Err(Error::TooShort),
                            }
                        }
                        None => Err(Error::TooShort),
                    }
                }
                None => Err(Error::TooShort),
            },
            None => Err(Error::TooShort),
        }
    }
}
