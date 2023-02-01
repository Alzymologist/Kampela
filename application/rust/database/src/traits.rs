use sled::IVec;

use kampela_common::Encryption;

use crate::error::ErrorCompanion;

pub trait DbKey: Sized {
    fn from_db_key(database_key: &IVec) -> Result<Self, ErrorCompanion>;
    fn as_db_key(&self) -> Vec<u8>;
    fn show(&self) -> String;
}

pub trait DbStorage: Sized {
    fn from_db_entry(database_entry: &(IVec, IVec)) -> Result<Self, ErrorCompanion>;
    fn db_key(&self) -> Vec<u8>;
    fn db_value(&self) -> Vec<u8>;
    fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion>;
}

pub trait FromQr: Sized {
    fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion>;
}

pub trait FromQrAndDb: Sized {
    fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
        db_path: &str,
    ) -> Result<Self, ErrorCompanion>;
}
