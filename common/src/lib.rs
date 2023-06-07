#![deny(unused_crate_dependencies)]
#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

use frame_metadata::RuntimeMetadataV14;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ByteArray, H256};

#[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq)]
pub enum Encryption {
    #[codec(index = 0)]
    Ed25519,

    #[codec(index = 1)]
    Sr25519,

    #[codec(index = 2)]
    Ecdsa,
}

impl Encryption {
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

#[derive(Debug, Decode, Encode)]
pub struct TransferData {
    pub encoded_data: Vec<u8>,
    pub companion_signature: Vec<u8>,
    pub companion_public_key: Vec<u8>,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub enum TransmittableContent {
    Bytes(Bytes),
    Derivation(DerivationInfo),
    SignableTransaction(Transaction),
    Specs(SpecsValue),
    SpecsSet(Vec<SpecsValue>),
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub enum MultiSigner {
    Ed25519(sp_core::ed25519::Public),
    Sr25519(sp_core::sr25519::Public),
    Ecdsa(sp_core::ecdsa::Public),
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub enum MultiSignature {
    Ed25519(sp_core::ed25519::Signature),
    Sr25519(sp_core::sr25519::Signature),
    Ecdsa(sp_core::ecdsa::Signature),
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct SpecsValue {
    pub specs: Specs,
    pub specs_signer: MultiSigner,
    pub specs_signature: MultiSignature,
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct SpecsKey {
    pub encryption: Encryption,
    pub genesis_hash: H256,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct Bytes {
    pub bytes_uncut: Vec<u8>,
    pub signer: MultiSigner,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct Transaction {
    pub genesis_hash: H256,
    pub meta_v14: RuntimeMetadataV14,
    pub meta_signature: MultiSignature,
    pub signable_transaction: Vec<u8>,
    pub signer: MultiSigner,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct DerivationInfo {
    pub chains: Vec<SpecsKey>,
    pub cut_path: String,
    pub has_pwd: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encryption() {
        assert_eq!(
            Encryption::decode(&mut [0].as_slice()).unwrap(),
            Encryption::Ed25519
        );
        assert_eq!(
            Encryption::decode(&mut [1].as_slice()).unwrap(),
            Encryption::Sr25519
        );
        assert_eq!(
            Encryption::decode(&mut [2].as_slice()).unwrap(),
            Encryption::Ecdsa
        );
        assert!(Encryption::decode(&mut [3].as_slice()).is_err());
    }
}
