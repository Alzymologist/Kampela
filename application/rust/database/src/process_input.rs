//! Basic processing of QR inputs.

use lt_codes::encoder::Encoder;
use parity_scale_codec::{Decode, Encode};
use sp_core::H256;
use std::{convert::TryInto, sync::{Arc, RwLock}};

use kampela_common::{
    Bytes, DerivationInfo, Encryption, MultiSigner, SpecsKey, SpecsValue, Transaction,
    TransmittableContent,
};

use crate::error::ErrorCompanion;
use crate::sign_with_companion::{SignByCompanion, SignatureMaker};
use crate::storage::{MetadataStorage, MetadataValue};
use crate::traits::{DbStorage, FromQr, FromQrAndDb};

pub const PREFIX_SUBSTRATE: u8 = 0x53;

pub const ID_SIGNABLE: &[u8] = &[0x00, 0x02];
pub const ID_BYTES: u8 = 0x03;
pub const ID_METADATA: u8 = 0x80;
pub const ID_SPECS: u8 = 0xc1;

impl FromQrAndDb for Transaction {
    fn from_payload_prelude_cut(
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
                meta_v14: metadata_value.metadata,
                meta_signature: metadata_value.signature,
                signable_transaction,
                signer,
            })
        } else {
            Err(ErrorCompanion::TooShort)
        }
    }
}

impl FromQr for Bytes {
    fn from_payload_prelude_cut(
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
    Transmit(Transmit),
}

#[derive(Debug)]
pub struct Transmit {
    data_with_signature: Vec<u8>,
    encoder: RwLock<Encoder>,
}

#[derive(Debug)]
pub struct Transmittable {
    content: TransmittableContent,
    signature_maker: Box<dyn SignByCompanion>,
}

impl Transmittable {
    pub fn into_transmit(self) -> Result<Transmit, ErrorCompanion> {
        let encoded_data = self.content.encode();
        let signature_maker = SignatureMaker::new(self.signature_maker);
        let data_with_signature = signature_maker.signed_data(encoded_data);
        let encoder = Encoder::init(&data_with_signature).map_err(|_| ErrorCompanion::LTError)?;
        Ok(Transmit{
            data_with_signature,
            encoder: RwLock::new(encoder),
        })
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
                let encryption = Encryption::decode(&mut &prelude[1..2])
                    .map_err(|_| ErrorCompanion::UnknownSigningAlgorithm(prelude[1]))?;
                match prelude[2] {
                    a if ID_SIGNABLE.contains(&a) => {
                        let transaction =
                            Transaction::from_payload_prelude_cut(payload, &encryption, db_path)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::SignableTransaction(transaction),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_transmit()?))
                    }
                    ID_BYTES => {
                        let bytes = Bytes::from_payload_prelude_cut(payload, &encryption)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::Bytes(bytes),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_transmit()?))
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
                        Ok(Self::Transmit(transmittable.into_transmit()?))
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
        let derivation = DerivationInfo {
            chains: specs_key_set
                .iter()
                .map(|a| a.as_ref().to_owned())
                .collect(),
            cut_path,
            has_pwd,
        };
        let transmittable = Transmittable {
            content: TransmittableContent::Derivation(derivation),
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_transmit()?))
    }

    pub fn new_specs_set(
        specs_values_set: Vec<Arc<SpecsValue>>,
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let specs_values = specs_values_set
            .iter()
            .map(|a| a.as_ref().to_owned())
            .collect();
        let transmittable = Transmittable {
            content: TransmittableContent::SpecsSet(specs_values),
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_transmit()?))
    }

    pub fn make_packet(self: &Arc<Self>) -> Option<Vec<u8>> {
        match self.as_ref() {
            Action::Success => None,
            Action::Transmit(transmit) => {
                let mut encoder = match transmit.encoder.write() {
                    Ok(a) => a,
                    Err(_) => return None,
                };
                encoder.make_packet(&transmit.data_with_signature).ok().map(|packet| packet.serialize().to_vec())
            },
        }
    }
}
