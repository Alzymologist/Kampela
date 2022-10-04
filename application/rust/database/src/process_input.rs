//! Basic processing of QR inputs.

use frame_metadata::RuntimeMetadataV14;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ByteArray, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use std::convert::TryInto;

use crate::error::Error;
use crate::nfc_fountain::pack_nfc;
use crate::storage::MetadataStorage;
use crate::utils::{read_meta, write_meta};

pub const PREFIX_SUBSTRATE: u8 = 0x53;

pub const ENCRYPTION_ED25519: u8 = 0x00;
pub const ENCRYPTION_SR25519: u8 = 0x01;
pub const ENCRYPTION_ECDSA: u8 = 0x02;

pub const ID_SIGNABLE: &[u8] = &[0x00, 0x02];
pub const ID_BYTES: u8 = 0x03;
pub const ID_METADATA: u8 = 0x80;
pub const ID_SPECS: u8 = 0xc1;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Success,
    Transmit { packets: Vec<Vec<u8>> },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Encryption {
    Ed25519,
    Sr25519,
    Ecdsa,
}

impl Encryption {
    pub fn from_symbol(symbol: u8) -> Result<Self, Error> {
        match symbol {
            ENCRYPTION_ED25519 => Ok(Encryption::Ed25519),
            ENCRYPTION_SR25519 => Ok(Encryption::Sr25519),
            ENCRYPTION_ECDSA => Ok(Encryption::Ecdsa),
            a => Err(Error::UnknownSigningAlgorithm(a)),
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

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct Transaction {
    pub genesis_hash: H256,
    pub meta_v14: RuntimeMetadataV14,
    pub meta_signature: MultiSignature,
    pub signable_transaction: Vec<u8>,
    pub signer: MultiSigner,
}

impl Transaction {
    pub fn transmittable(&self) -> Result<Vec<Vec<u8>>, Error> {
        let data = self.encode();
        // data must be signed here
        pack_nfc(&data)
    }
}

pub fn process_qr_input(mut payload: &[u8], db_path: &str) -> Result<Action, Error> {
    match payload.get(..3) {
        Some(prelude) => {
            payload = &payload[3..];
            if prelude[0] != PREFIX_SUBSTRATE {
                return Err(Error::NotSubstrate);
            }
            let encryption = Encryption::from_symbol(prelude[1])?;
            match prelude[2] {
                a if ID_SIGNABLE.contains(&a) => {
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
                        let signable_transaction =
                            payload[..payload.len() - H256::len_bytes()].to_vec();
                        let transaction = Transaction {
                            genesis_hash,
                            meta_v14: metadata_value.metadata(),
                            meta_signature: metadata_value.signature(),
                            signable_transaction,
                            signer,
                        };
                        Ok(Action::Transmit {
                            packets: transaction.transmittable()?,
                        })
                    } else {
                        Err(Error::TooShort)
                    }
                }
                ID_METADATA => {
                    let metadata_storage =
                        MetadataStorage::from_payload_prelude_cut(payload, &encryption)?;
                    write_meta(db_path, metadata_storage)?;
                    Ok(Action::Success)
                }
                a => Err(Error::UnknownPayloadType(a)),
            }
        }
        None => Err(Error::TooShort),
    }
}