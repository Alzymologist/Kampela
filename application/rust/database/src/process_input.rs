//! Basic processing of QR inputs.

use frame_metadata::RuntimeMetadataV14;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ByteArray, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use std::{convert::TryInto, sync::Arc};

use crate::derivation::DerivationInfo;
use crate::error::ErrorCompanion;
use crate::nfc_fountain::pack_nfc;
use crate::sign_with_companion::{SignByCompanion, SignatureMaker};
use crate::storage::{MetadataStorage, MetadataValue, SpecsKey, SpecsValue};

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

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct Transaction {
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
                genesis_hash,
                meta_v14: metadata_value.metadata(),
                meta_signature: metadata_value.signature(),
                signable_transaction,
                signer,
            })
        } else {
            Err(ErrorCompanion::TooShort)
        }
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
}

#[derive(Debug)]
pub enum Action {
    Success,
    Transmit(Vec<Vec<u8>>),
}

#[derive(Debug)]
pub struct Transmittable {
    content: TransmittableContent,
    signature_maker: Box<dyn SignByCompanion>,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub enum TransmittableContent {
    Bytes(Bytes),
    Derivation(DerivationInfo),
    SignableTransaction(Transaction),
    Specs(SpecsValue),
    SpecsSet(Vec<Arc<SpecsValue>>),
}

impl Transmittable {
    pub fn into_packets(self) -> Result<Vec<Vec<u8>>, ErrorCompanion> {
        let encoded_data = self.content.encode();
        let signature_maker = SignatureMaker::new(self.signature_maker);
        pack_nfc(&signature_maker.signed_data(encoded_data))
    }
}

impl Action {
    pub fn new_payload(
        mut payload: &[u8],
        db_path: &str,
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
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
                            Transaction::from_payload_prelude_cut(payload, &encryption, db_path)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::SignableTransaction(transaction),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_packets()?))
                    }
                    ID_BYTES => {
                        let bytes = Bytes::from_payload_prelude_cut(payload, &encryption)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::Bytes(bytes),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_packets()?))
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
                        specs_value.write_in_db(db_path)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::Specs(specs_value),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_packets()?))
                    }
                    a => Err(ErrorCompanion::UnknownPayloadType(a)),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }

    pub fn new_derivation(
        specs_key_set: Vec<Arc<SpecsKey>>,
        cut_path: String,
        has_pwd: bool,
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let derivation = DerivationInfo::new(specs_key_set, cut_path, has_pwd);
        let transmittable = Transmittable {
            content: TransmittableContent::Derivation(derivation),
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_packets()?))
    }

    pub fn new_specs_set(
        specs_values_set: Vec<Arc<SpecsValue>>,
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let transmittable = Transmittable {
            content: TransmittableContent::SpecsSet(specs_values_set),
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_packets()?))
    }

    pub fn as_transmittable(&self) -> Option<Vec<Vec<u8>>> {
        match &self {
            Action::Success => None,
            Action::Transmit(packets) => Some(packets.to_owned()),
        }
    }
}
