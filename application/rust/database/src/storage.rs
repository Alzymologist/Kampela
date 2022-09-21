//! Keys and corresponding values in companion database.
use frame_metadata::{RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use sp_core::H256;
use sp_runtime::MultiSignature;
use std::convert::TryInto;
use substrate_parser::compacts::find_compact;

use crate::error::Error;
use crate::process_input::Encryption;

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

impl MetadataStorage {
    pub fn from_payload_prelude_cut(
        mut payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, Error> {
        payload = match payload.get(encryption.key_length()..) {
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
