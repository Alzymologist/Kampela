//! Errors occuring in companion.
use sp_core::H256;
use substrate_parser::error::MetaVersionError;

use crate::process_input::Encryption;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum ErrorCompanion {
    #[error("Internal database error: {0}")]
    DbInternal(sled::Error),

    #[error("Mismatch in encryption in specs storage.")]
    DbSpecsEncryptionMismatch { key: Encryption, value: Encryption },

    #[error("Mismatch in genesis hash in specs storage.")]
    DbSpecsHashMismatch { key: H256, value: H256 },

    #[error("Metadata storage key got damaged in the database and could not be decoded.")]
    DecodeDbMetadataKey,

    #[error("Metadata storage value got damaged in the database and could not be decoded.")]
    DecodeDbMetadataValue,

    #[error("Specs storage key got damaged in the database and could not be decoded.")]
    DecodeDbSpecsKey,

    #[error("Specs storage value got damaged in the database and could not be decoded.")]
    DecodeDbSpecsValue,

    #[error("Metadata from scanned QR could not be decoded.")]
    MetadataQrDecode,

    #[error("Received metadata QR payload has unexpected structure. Can not find compact to cut payload into parts.")]
    MetadataQrUnexpectedStructure,

    #[error("Error determining metadata version. {0}")]
    MetadataVersion(MetaVersionError),

    #[error("No metadata entries for genesis hash {} in the database.", hex::encode(.0))]
    NoMetadata(H256),

    #[error("Metadata from scanned QR does not start with expected b`meta` prefix.")]
    NoMetaPrefixQr,

    #[error("Received QR payload is not a Substrate one.")]
    NotSubstrate,

    #[error("Metadata in received QR payload is not V14 and is not supported.")]
    OnlyV14SupportedQr,

    #[error("Poisoned lock on uniffi interface.")]
    PoisonedLock,

    #[error("Specs from scanned QR could not be decoded.")]
    SpecsQrDecode,

    #[error("Input size too large to form NFC. Please file a ticket if you see this.")]
    TooLargeInputForNFC,

    #[error("Received QR payload is too short.")]
    TooShort,

    #[error("Unexpected payload type, 0x{}", hex::encode([*.0]))]
    UnknownPayloadType(u8),

    #[error("Unexpected signing algorithm, 0x{}", hex::encode([*.0]))]
    UnknownSigningAlgorithm(u8),
}
