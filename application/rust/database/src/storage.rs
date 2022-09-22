//! Keys and corresponding values in companion database.
use frame_metadata::{RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::{Decode, Encode};
use sled::{open, Db, IVec, Tree};
use sp_core::H256;
use sp_runtime::{MultiSignature, MultiSigner};
use std::convert::TryInto;
use substrate_parser::compacts::find_compact;

use crate::error::Error;
use crate::nfc_fountain::pack_nfc;
use crate::process_input::{Action, Encryption};

fn open_db(db_path: &str) -> Result<Db, Error> {
    open(db_path).map_err(Error::DbInternal)
}

fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree, Error> {
    database.open_tree(tree_name).map_err(Error::DbInternal)
}

/// Tree name for metadata storage
pub const METADATA: &[u8] = b"metadata";

/// Tree name for specs storage
pub const SPECS: &[u8] = b"specs";

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
    pub fn read_from_db(db_path: &str, genesis_hash: H256) -> Result<Self, Error> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        let metadata_key = MetadataKey::new(genesis_hash);
        match metadata_tree.get(metadata_key.as_db_key()) {
            Ok(Some(a)) => Self::from_db_value(&a),
            Ok(None) => Err(Error::NoMetadata(genesis_hash)),
            Err(e) => Err(Error::DbInternal(e)),
        }
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
    pub fn write_in_db(&self, db_path: &str) -> Result<(), Error> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        metadata_tree
            .insert(self.key.as_db_key(), self.value.as_db_value())
            .map_err(Error::DbInternal)?;
        Ok(())
    }
}

pub struct SpecsKey(SpecsKeyContent);

#[derive(Decode, Encode)]
struct SpecsKeyContent {
    encryption: Encryption,
    genesis_hash: H256,
}

impl SpecsKey {
    pub fn new(encryption: Encryption, genesis_hash: H256) -> Self {
        Self(SpecsKeyContent {
            encryption,
            genesis_hash,
        })
    }
    pub fn from_db_key(database_key: &IVec) -> Result<Self, Error> {
        Ok(Self(
            SpecsKeyContent::decode(&mut &database_key[..]).map_err(|_| Error::DecodeDbSpecsKey)?,
        ))
    }
    pub fn as_db_key(&self) -> Vec<u8> {
        self.0.encode()
    }
    pub fn hash(&self) -> H256 {
        self.0.genesis_hash.to_owned()
    }
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct Specs {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: H256,
    pub logo: String,
    pub name: String,
    pub order: u8,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

pub struct SpecsValue(SpecsValueContent);

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct SpecsValueContent {
    specs: Specs,
    specs_signer: MultiSigner,
    specs_signature: MultiSignature,
}

impl SpecsValue {
    pub fn from_db_value(database_value: &IVec) -> Result<Self, Error> {
        Ok(Self(
            SpecsValueContent::decode(&mut &database_value[..])
                .map_err(|_| Error::DecodeDbSpecsValue)?,
        ))
    }
    pub fn read_from_db(
        db_path: &str,
        encryption: Encryption,
        genesis_hash: H256,
    ) -> Result<Self, Error> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        let specs_key = SpecsKey::new(encryption, genesis_hash);
        match specs_tree.get(specs_key.as_db_key()) {
            Ok(Some(a)) => {
                let specs_value = Self::from_db_value(&a)?;
                if specs_value.specs().encryption != encryption {
                    return Err(Error::DbSpecsEncryptionMismatch {
                        key: encryption,
                        value: specs_value.specs().encryption,
                    });
                }
                if specs_value.specs().genesis_hash != genesis_hash {
                    return Err(Error::DbSpecsHashMismatch {
                        key: genesis_hash,
                        value: specs_value.specs().genesis_hash,
                    });
                }
                Ok(specs_value)
            }
            Ok(None) => Err(Error::NoSpecs(genesis_hash)),
            Err(e) => Err(Error::DbInternal(e)),
        }
    }
    pub fn as_db_value(&self) -> Vec<u8> {
        self.0.encode()
    }
    pub fn specs(&self) -> Specs {
        self.0.specs.to_owned()
    }
    pub fn signature(&self) -> MultiSignature {
        self.0.specs_signature.to_owned()
    }
    pub fn signer(&self) -> MultiSigner {
        self.0.specs_signer.to_owned()
    }
    pub fn from_payload_prelude_cut(
        mut payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, Error> {
        let specs_signer = match payload.get(0..encryption.key_length()) {
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
        let length_info =
            find_compact::<u32>(payload).map_err(|_| Error::MetadataQrUnexpectedStructure)?;
        let encoded_specs_length = length_info.compact as usize;
        match length_info.start_next_unit {
            Some(start) => match payload.get(..start + encoded_specs_length) {
                Some(encoded_specs_slice) => {
                    let specs = Specs::decode(&mut &encoded_specs_slice[..])
                        .map_err(|_| Error::SpecsQrDecode)?;
                    payload = &payload[start + encoded_specs_length..];
                    match payload.get(..encryption.signature_length()) {
                        Some(signature_slice) => {
                            let specs_signature = match encryption {
                                Encryption::Ed25519 => MultiSignature::Ed25519(
                                    signature_slice.try_into().expect("stable known length"),
                                ),
                                Encryption::Sr25519 => MultiSignature::Sr25519(
                                    signature_slice.try_into().expect("stable known length"),
                                ),
                                Encryption::Ecdsa => MultiSignature::Ecdsa(
                                    signature_slice.try_into().expect("stable known length"),
                                ),
                            };
                            Ok(Self(SpecsValueContent {
                                specs,
                                specs_signer,
                                specs_signature,
                            }))
                        }
                        None => Err(Error::TooShort),
                    }
                }
                None => Err(Error::TooShort),
            },
            None => Err(Error::TooShort),
        }
    }
    pub fn write_in_db(&self, db_path: &str) -> Result<(), Error> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        specs_tree
            .insert(
                SpecsKey::new(self.specs().encryption, self.specs().genesis_hash).as_db_key(),
                self.as_db_value(),
            )
            .map_err(Error::DbInternal)?;
        Ok(())
    }
    pub fn transmit(&self) -> Result<Action, Error> {
        let data = self.0.encode();
        // data must be signed here
        Ok(Action::TransmitSpecs {
            packets: pack_nfc(&data)?,
        })
    }
}
