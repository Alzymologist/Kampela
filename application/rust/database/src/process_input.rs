//! Basic processing of QR inputs.

use frame_metadata::RuntimeMetadataV14;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ByteArray, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use std::convert::TryInto;

use crate::error::ErrorCompanion;
use crate::nfc_fountain::pack_nfc;
use crate::sign_with_companion::{SignedByCompanion, SignedData};
use crate::storage::{MetadataStorage, MetadataValue, SpecsValue};

pub const PREFIX_SUBSTRATE: u8 = 0x53;

pub const ENCRYPTION_ED25519: u8 = 0x00;
pub const ENCRYPTION_SR25519: u8 = 0x01;
pub const ENCRYPTION_ECDSA: u8 = 0x02;

pub const ID_SIGNABLE: &[u8] = &[0x00, 0x02];
pub const ID_BYTES: u8 = 0x03;
pub const ID_METADATA: u8 = 0x80;
pub const ID_SPECS: u8 = 0xc1;

#[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq)]
pub enum Encryption {
    Ed25519,
    Sr25519,
    Ecdsa,
}

impl Encryption {
    pub fn from_symbol(symbol: u8) -> Result<Self, ErrorCompanion> {
        match symbol {
            ENCRYPTION_ED25519 => Ok(Encryption::Ed25519),
            ENCRYPTION_SR25519 => Ok(Encryption::Sr25519),
            ENCRYPTION_ECDSA => Ok(Encryption::Ecdsa),
            a => Err(ErrorCompanion::UnknownSigningAlgorithm(a)),
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

#[derive(Debug)]
pub struct Transaction {
    core: TransactionCore,
    signed_data: Box<dyn SignedByCompanion>,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct TransactionCore {
    genesis_hash: H256,
    meta_v14: RuntimeMetadataV14,
    meta_signature: MultiSignature,
    signable_transaction: Vec<u8>,
    signer: MultiSigner,
}

impl Transaction {
    pub fn from_payload_prelude_cut(
        mut payload: &[u8],
        encryption: &Encryption,
        db_path: &str,
        signed_data: Box<dyn SignedByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
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
            None => return Err(ErrorCompanion::TooShort),
        };
        if payload.len() >= H256::len_bytes() {
            let genesis_hash = H256(
                payload[payload.len() - H256::len_bytes()..]
                    .try_into()
                    .expect("stable known length"),
            );
            let metadata_value = MetadataValue::read_from_db(db_path, genesis_hash)?;
            let signable_transaction = payload[..payload.len() - H256::len_bytes()].to_vec();
            Ok(Self {
                core: TransactionCore{
                    genesis_hash,
                    meta_v14: metadata_value.metadata(),
                    meta_signature: metadata_value.signature(),
                    signable_transaction,
                    signer,
                },
                signed_data,
            })
        } else {
            Err(ErrorCompanion::TooShort)
        }
    }
    pub fn data(&self) -> Vec<u8> {
        self.core.encode()
    }
    pub fn transmit(self) -> Result<Action, ErrorCompanion> {
        let bytes = self.data();
        let signed_data = SignedData::new(self.signed_data);
        Ok(Action::TransmitSignable {
            packets: pack_nfc(&signed_data.sign(bytes))?,
        })
    }
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct Bytes {
    bytes_uncut: Vec<u8>,
    signer: MultiSigner,
}

impl Bytes {
    pub fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        match payload.get(0..encryption.key_length()) {
            Some(public_key_slice) => {
                let bytes_uncut = payload[encryption.key_length()..].to_vec();
                let signer = match encryption {
                    Encryption::Ed25519 => MultiSigner::Ed25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Sr25519 => MultiSigner::Sr25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Ecdsa => MultiSigner::Ecdsa(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                };
                Ok(Self {
                    bytes_uncut,
                    signer,
                })
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }
    pub fn data(&self) -> Vec<u8> {
        self.encode()
    }
    pub fn transmit(&self) -> Result<Action, ErrorCompanion> {
        let data = self.encode();
        // data must be signed here
        Ok(Action::TransmitBytes {
            packets: pack_nfc(&data)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Success,
    TransmitBytes { packets: Vec<Vec<u8>> },
    TransmitSignable { packets: Vec<Vec<u8>> },
    TransmitSpecs { packets: Vec<Vec<u8>> },
}

impl Action {
    pub fn new(mut payload: &[u8], db_path: &str, signed_data: Box<dyn SignedByCompanion>) -> Result<Self, ErrorCompanion> {
        match payload.get(..3) {
            Some(prelude) => {
                payload = &payload[3..];
                if prelude[0] != PREFIX_SUBSTRATE {
                    return Err(ErrorCompanion::NotSubstrate);
                }
                let encryption = Encryption::from_symbol(prelude[1])?;
                match prelude[2] {
                    a if ID_SIGNABLE.contains(&a) => {
                        let transaction =
                            Transaction::from_payload_prelude_cut(payload, &encryption, db_path, signed_data)?;
                        transaction.transmit()
                    }
                    ID_BYTES => {
                        let bytes = Bytes::from_payload_prelude_cut(payload, &encryption)?;
                        bytes.transmit()
                    }
                    ID_METADATA => {
                        let metadata_storage =
                            MetadataStorage::from_payload_prelude_cut(payload, &encryption)?;
                        metadata_storage.write_in_db(db_path)?;
                        Ok(Self::Success)
                    }
                    ID_SPECS => {
                        let specs_value =
                            SpecsValue::from_payload_prelude_cut(payload, &encryption)?;
                        let action = specs_value.transmit()?;
                        specs_value.write_in_db(db_path)?;
                        Ok(action)
                    }
                    a => Err(ErrorCompanion::UnknownPayloadType(a)),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }
}
