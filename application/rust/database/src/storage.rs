//! Keys and corresponding values in companion database.
use frame_metadata::v14::RuntimeMetadataV14;
use parity_scale_codec::{Decode, Encode};
use sled::{open, Db, IVec, Tree};
use sp_core::H256;
use std::{
    convert::TryInto,
    sync::{Arc, RwLock},
};
use substrate_parser::{
    compacts::find_compact,
    traits::AsMetadata,
};

use kampela_common::{Encryption, MultiSignature, MultiSigner, Specs, SpecsKey, SpecsValue};

use crate::error::ErrorCompanion;
use crate::traits::{DbKey, DbStorage, FromQr};

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

#[derive(Decode, Encode)]
pub struct MetadataKey {
    pub genesis_hash: H256,
}

impl DbKey for MetadataKey {
    fn from_db_key(database_key: &IVec) -> Result<Self, ErrorCompanion> {
        Self::decode(&mut &database_key[..]).map_err(|_| ErrorCompanion::DecodeDbMetadataKey)
    }
    fn as_db_key(&self) -> Vec<u8> {
        self.encode()
    }
    fn show(&self) -> String {
        hex::encode(self.genesis_hash)
    }
}

#[derive(Decode, Encode)]
pub struct MetadataValue {
    pub metadata: RuntimeMetadataV14,
}

impl MetadataValue {
    pub fn from_db_value(database_value: &IVec) -> Result<Self, ErrorCompanion> {
        Self::decode(&mut &database_value[..]).map_err(|_| ErrorCompanion::DecodeDbMetadataValue)
    }
    pub fn try_read_from_tree(
        metadata_tree: &Tree,
        genesis_hash: H256,
    ) -> Result<Option<Self>, ErrorCompanion> {
        let metadata_key = MetadataKey { genesis_hash };
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
        self.encode()
    }
}

pub struct MetadataStorage {
    pub key: MetadataKey,
    pub value: MetadataValue,
}

impl DbStorage for MetadataStorage {
    fn from_db_entry(
        (database_key, database_value): &(IVec, IVec),
    ) -> Result<Self, ErrorCompanion> {
        let key = MetadataKey::from_db_key(database_key)?;
        let value = MetadataValue::from_db_value(database_value)?;
        Ok(Self { key, value })
    }
    fn db_key(&self) -> Vec<u8> {
        self.key.as_db_key()
    }
    fn db_value(&self) -> Vec<u8> {
        self.value.as_db_value()
    }
    fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        metadata_tree
            .insert(self.key.as_db_key(), self.value.as_db_value())
            .map_err(ErrorCompanion::DbInternal)?;
        Ok(())
    }
}

impl FromQr for MetadataStorage {
    fn from_payload_prelude_cut(
        payload: &[u8],
        _encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        if payload.len() <= H256::len_bytes() {
            Err(ErrorCompanion::TooShort)
        } else {
            let genesis_hash = H256(
                payload[payload.len() - H256::len_bytes()..]
                    .try_into()
                    .expect("stable known length"),
            );
            let metadata = RuntimeMetadataV14::decode(
                &mut &payload[..payload.len() - H256::len_bytes()],
            )
            .map_err(|_| ErrorCompanion::MetadataQrDecode)?;
            Ok(Self {
                key: MetadataKey { genesis_hash },
                value: MetadataValue { metadata },
            })
        }
    }
}

impl DbKey for SpecsKey {
    fn from_db_key(database_key: &IVec) -> Result<Self, ErrorCompanion> {
        Self::decode(&mut &database_key[..]).map_err(|_| ErrorCompanion::DecodeDbSpecsKey)
    }
    fn as_db_key(&self) -> Vec<u8> {
        self.encode()
    }
    fn show(&self) -> String {
        hex::encode(self.as_db_key())
    }
}

impl DbStorage for SpecsValue {
    fn from_db_entry(
        (database_key, database_value): &(IVec, IVec),
    ) -> Result<Self, ErrorCompanion> {
        let key = SpecsKey::from_db_key(database_key)?;
        let value = Self::decode(&mut &database_value[..])
            .map_err(|_| ErrorCompanion::DecodeDbSpecsValue)?;
        if value.specs.encryption != key.encryption {
            return Err(ErrorCompanion::DbSpecsEncryptionMismatch {
                key: key.encryption,
                value: value.specs.encryption,
            });
        }
        if value.specs.genesis_hash != key.genesis_hash {
            return Err(ErrorCompanion::DbSpecsHashMismatch {
                key: key.genesis_hash,
                value: value.specs.genesis_hash,
            });
        }
        Ok(value)
    }
    fn db_key(&self) -> Vec<u8> {
        SpecsKey {
            encryption: self.specs.encryption,
            genesis_hash: self.specs.genesis_hash,
        }
        .as_db_key()
    }
    fn db_value(&self) -> Vec<u8> {
        self.encode()
    }
    fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        specs_tree
            .insert(self.db_key(), self.db_value())
            .map_err(ErrorCompanion::DbInternal)?;
        Ok(())
    }
}

impl FromQr for SpecsValue {
    fn from_payload_prelude_cut(
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
        let length_info = find_compact::<u32, _, _>(&payload, &mut (), position)
            .map_err(|_| ErrorCompanion::SpecsQrUnexpectedStructure)?;
        let encoded_specs_length = length_info.compact as usize;
        position = length_info.start_next_unit;
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
        database_entry: (IVec, IVec),
        metadata_tree: &Tree,
    ) -> Result<Self, ErrorCompanion> {
        let value = SpecsValue::from_db_entry(&database_entry)?;
        let key = SpecsKey {
            encryption: value.specs.encryption,
            genesis_hash: value.specs.genesis_hash,
        };
        let metadata_version =
            match MetadataValue::try_read_from_tree(metadata_tree, key.genesis_hash)? {
                Some(metadata_value) => Some(
                    <RuntimeMetadataV14 as AsMetadata<()>>::version_printed(
                        &metadata_value.metadata,
                    )
                    .map_err(ErrorCompanion::MetadataVersion)?,
                ),
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
        self.value.specs.title.to_owned()
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
