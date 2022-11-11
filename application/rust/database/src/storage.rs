//! Keys and corresponding values in companion database.
use frame_metadata::{RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::{Decode, Encode};
use sled::{open, Db, IVec, Tree};
use sp_core::H256;
use sp_runtime::{MultiSignature, MultiSigner};
use std::{
    convert::TryInto,
    sync::{Arc, RwLock},
};
use substrate_parser::{compacts::find_compact, CheckedMetadata};

use crate::error::ErrorCompanion;
use crate::process_input::Encryption;

fn open_db(db_path: &str) -> Result<Db, ErrorCompanion> {
    open(db_path).map_err(ErrorCompanion::DbInternal)
}

fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree, ErrorCompanion> {
    database
        .open_tree(tree_name)
        .map_err(ErrorCompanion::DbInternal)
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
    pub fn from_db_key(database_key: &IVec) -> Result<Self, ErrorCompanion> {
        Ok(Self(
            MetadataKeyContent::decode(&mut &database_key[..])
                .map_err(|_| ErrorCompanion::DecodeDbMetadataKey)?,
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
    pub fn from_db_value(database_value: &IVec) -> Result<Self, ErrorCompanion> {
        Ok(Self(
            MetadataValueContent::decode(&mut &database_value[..])
                .map_err(|_| ErrorCompanion::DecodeDbMetadataValue)?,
        ))
    }
    pub fn try_read_from_tree(
        metadata_tree: &Tree,
        genesis_hash: H256,
    ) -> Result<Option<Self>, ErrorCompanion> {
        let metadata_key = MetadataKey::new(genesis_hash);
        match metadata_tree.get(metadata_key.as_db_key()) {
            Ok(Some(a)) => Self::from_db_value(&a).map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(ErrorCompanion::DbInternal(e)),
        }
    }
    pub fn read_from_tree(
        metadata_tree: &Tree,
        genesis_hash: H256,
    ) -> Result<Self, ErrorCompanion> {
        match Self::try_read_from_tree(metadata_tree, genesis_hash)? {
            Some(a) => Ok(a),
            None => Err(ErrorCompanion::NoMetadata(genesis_hash)),
        }
    }
    pub fn read_from_db(db_path: &str, genesis_hash: H256) -> Result<Self, ErrorCompanion> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        Self::read_from_tree(&metadata_tree, genesis_hash)
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
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        let mut position = encryption.key_length();
        let length_info = find_compact::<u32>(payload, position)
            .map_err(|_| ErrorCompanion::MetadataQrUnexpectedStructure)?;
        let meta_length = length_info.compact as usize;
        position += length_info.start_next_unit;
        match payload.get(position..position + meta_length) {
            Some(meta_slice) => {
                if !meta_slice.starts_with(&[109, 101, 116, 97]) {
                    return Err(ErrorCompanion::NoMetaPrefixQr);
                }
                let meta_decoded = RuntimeMetadata::decode(&mut &meta_slice[4..])
                    .map_err(|_| ErrorCompanion::MetadataQrDecode)?;
                let metadata = match meta_decoded {
                    RuntimeMetadata::V14(metadata) => metadata,
                    _ => return Err(ErrorCompanion::OnlyV14SupportedQr),
                };
                position += meta_length;
                match payload.get(position..position + H256::len_bytes()) {
                    Some(hash_slice) => {
                        let hash = H256(hash_slice.try_into().expect("stable known length"));
                        position += H256::len_bytes();
                        match payload.get(position..position + encryption.signature_length()) {
                            Some(signature_slice) => {
                                let signature = match encryption {
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
                                Ok(Self {
                                    key: MetadataKey::new(hash),
                                    value: MetadataValue(MetadataValueContent {
                                        metadata,
                                        signature,
                                    }),
                                })
                            }
                            None => Err(ErrorCompanion::TooShort),
                        }
                    }
                    None => Err(ErrorCompanion::TooShort),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }
    pub fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        metadata_tree
            .insert(self.key.as_db_key(), self.value.as_db_value())
            .map_err(ErrorCompanion::DbInternal)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecsKey(SpecsKeyContent);

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
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
    pub fn from_db_key(database_key: &IVec) -> Result<Self, ErrorCompanion> {
        Ok(Self(
            SpecsKeyContent::decode(&mut &database_key[..])
                .map_err(|_| ErrorCompanion::DecodeDbSpecsKey)?,
        ))
    }
    pub fn as_db_key(&self) -> Vec<u8> {
        self.0.encode()
    }
    pub fn encryption(&self) -> Encryption {
        self.0.encryption.to_owned()
    }
    pub fn hash(&self) -> H256 {
        self.0.genesis_hash.to_owned()
    }
    pub fn show(&self) -> String {
        hex::encode(self.as_db_key())
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
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct SpecsValue {
    specs: Specs,
    specs_signer: MultiSigner,
    specs_signature: MultiSignature,
}

impl SpecsValue {
    pub fn from_db_value(database_value: &IVec) -> Result<Self, ErrorCompanion> {
        Self::decode(&mut &database_value[..]).map_err(|_| ErrorCompanion::DecodeDbSpecsValue)
    }
    pub fn as_db_value(&self) -> Vec<u8> {
        self.encode()
    }
    pub fn specs(&self) -> Specs {
        self.specs.to_owned()
    }
    pub fn signature(&self) -> MultiSignature {
        self.specs_signature.to_owned()
    }
    pub fn signer(&self) -> MultiSigner {
        self.specs_signer.to_owned()
    }
    pub fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        let mut position = 0;
        let specs_signer = match payload.get(position..position + encryption.key_length()) {
            Some(public_key_slice) => {
                position += encryption.key_length();
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
            None => return Err(ErrorCompanion::TooShort),
        };
        let length_info = find_compact::<u32>(payload, position)
            .map_err(|_| ErrorCompanion::MetadataQrUnexpectedStructure)?;
        let encoded_specs_length = length_info.compact as usize;
        position += length_info.start_next_unit;
        match payload.get(position..position + encoded_specs_length) {
            Some(encoded_specs_slice) => {
                let specs = Specs::decode(&mut &encoded_specs_slice[..])
                    .map_err(|_| ErrorCompanion::SpecsQrDecode)?;
                position += encoded_specs_length;
                match payload.get(position..position + encryption.signature_length()) {
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
                        Ok(Self {
                            specs,
                            specs_signer,
                            specs_signature,
                        })
                    }
                    None => Err(ErrorCompanion::TooShort),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }
    pub fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        specs_tree
            .insert(
                SpecsKey::new(self.specs().encryption, self.specs().genesis_hash).as_db_key(),
                self.as_db_value(),
            )
            .map_err(ErrorCompanion::DbInternal)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SpecsSelectorElement {
    key: SpecsKey,
    value: SpecsValue,
    is_selected: bool,
    metadata_version: Option<String>,
}

impl SpecsSelectorElement {
    fn from_entry(
        (specs_key_db, specs_value_db): (IVec, IVec),
        metadata_tree: &Tree,
    ) -> Result<Self, ErrorCompanion> {
        let key = SpecsKey::from_db_key(&specs_key_db)?;
        let value = SpecsValue::from_db_value(&specs_value_db)?;
        if value.specs().encryption != key.encryption() {
            return Err(ErrorCompanion::DbSpecsEncryptionMismatch {
                key: key.encryption(),
                value: value.specs().encryption,
            });
        }
        if value.specs().genesis_hash != key.hash() {
            return Err(ErrorCompanion::DbSpecsHashMismatch {
                key: key.hash(),
                value: value.specs().genesis_hash,
            });
        }
        let metadata_version = match MetadataValue::try_read_from_tree(metadata_tree, key.hash())? {
            Some(metadata_value) => {
                let checked_metadata = CheckedMetadata::new(metadata_value.metadata())
                    .map_err(ErrorCompanion::MetadataVersion)?;
                Some(checked_metadata.version)
            }
            None => None,
        };
        Ok(Self {
            key,
            value,
            is_selected: false,
            metadata_version,
        })
    }
    fn toggle(&mut self) {
        self.is_selected = !self.is_selected;
    }
    fn make_selected(&mut self) {
        self.is_selected = true;
    }
    fn make_deselected(&mut self) {
        self.is_selected = false;
    }
    fn title(&self) -> String {
        self.value.specs().title
    }
    fn version(&self) -> Option<String> {
        self.metadata_version.to_owned()
    }
    fn is_selected(&self) -> bool {
        self.is_selected
    }
    fn key(&self) -> SpecsKey {
        self.key.to_owned()
    }
    fn value(&self) -> SpecsValue {
        self.value.to_owned()
    }
}

#[derive(Debug)]
pub struct SpecsSelector {
    selector: RwLock<Vec<SpecsSelectorElement>>,
}

impl SpecsSelector {
    pub fn new(db_path: &str) -> Result<Self, ErrorCompanion> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        let mut selector: Vec<SpecsSelectorElement> = Vec::new();
        for x in specs_tree.iter().flatten() {
            selector.push(SpecsSelectorElement::from_entry(x, &metadata_tree)?)
        }
        Ok(Self {
            selector: RwLock::new(selector),
        })
    }
    pub fn get_all_keys(&self) -> Result<Vec<Arc<SpecsKey>>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        Ok(selector.iter().map(|a| Arc::new(a.key())).collect())
    }
    pub fn collect_selected_keys(&self) -> Result<Vec<Arc<SpecsKey>>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        Ok(selector
            .iter()
            .filter(|a| a.is_selected())
            .map(|a| Arc::new(a.key()))
            .collect())
    }
    pub fn collect_selected_values(&self) -> Result<Vec<Arc<SpecsValue>>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        Ok(selector
            .iter()
            .filter(|a| a.is_selected())
            .map(|a| Arc::new(a.value()))
            .collect())
    }
    pub fn title(&self, key: &SpecsKey) -> Result<Option<String>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        let mut title = None;
        for element in selector.iter() {
            if &element.key() == key {
                title = Some(element.title());
                break;
            }
        }
        Ok(title)
    }
    pub fn version(&self, key: &SpecsKey) -> Result<Option<String>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        let mut version = None;
        for element in selector.iter() {
            if &element.key() == key {
                version = element.version();
                break;
            }
        }
        Ok(version)
    }
    pub fn is_selected(&self, key: &SpecsKey) -> Result<Option<bool>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        let mut title = None;
        for element in selector.iter() {
            if &element.key() == key {
                title = Some(element.is_selected());
                break;
            }
        }
        Ok(title)
    }
    pub fn toggle(self: &Arc<Self>, key: &SpecsKey) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        for element in selector.iter_mut() {
            if &element.key() == key {
                element.toggle();
                break;
            }
        }
        Ok(())
    }
    pub fn select_all(self: &Arc<Self>) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        for element in selector.iter_mut() {
            element.make_selected()
        }
        Ok(())
    }
    pub fn deselect_all(self: &Arc<Self>) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLock)?;
        for element in selector.iter_mut() {
            element.make_deselected()
        }
        Ok(())
    }
}
