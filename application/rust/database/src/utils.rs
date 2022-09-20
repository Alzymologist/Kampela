use sled::{open, Db, Tree};
use sp_core::H256;

use crate::error::Error;
use crate::storage::{MetadataKey, MetadataStorage, MetadataValue};

pub fn open_db(db_path: &str) -> Result<Db, Error> {
    open(db_path).map_err(Error::DbInternal)
}

pub fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree, Error> {
    database.open_tree(tree_name).map_err(Error::DbInternal)
}

/// Tree name for metadata storage
pub const METADATA: &[u8] = b"metadata";

pub fn write_meta(db_path: &str, metadata_payload: &[u8]) -> Result<(), Error> {
    let metadata_storage = MetadataStorage::from_metadata_qr(metadata_payload)?;
    let database = open_db(db_path)?;
    let metadata_tree = open_tree(&database, METADATA)?;
    metadata_tree
        .insert(
            metadata_storage.key.as_db_key(),
            metadata_storage.value.as_db_value(),
        )
        .map_err(Error::DbInternal)?;
    Ok(())
}

pub fn read_meta(db_path: &str, genesis_hash: H256) -> Result<MetadataValue, Error> {
    let database = open_db(db_path)?;
    let metadata_tree = open_tree(&database, METADATA)?;
    let metadata_key = MetadataKey::new(genesis_hash);
    match metadata_tree.get(metadata_key.as_db_key()) {
        Ok(Some(a)) => MetadataValue::from_db_value(&a),
        Ok(None) => Err(Error::NoMetadata(genesis_hash)),
        Err(e) => Err(Error::DbInternal(e)),
    }
}
