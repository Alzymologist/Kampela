use sp_core::H256;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Internal database error: {0}")]
    DbInternal(sled::Error),

    #[error("Metadata storage value got damaged in the database and could not be decoded.")]
    DecodeDbMetadataValue,

    #[error("Metadata storage key got damaged in the database and could not be decoded.")]
    DecodeDbMetadataKey,

    #[error("Metadata from scanned QR could not be decoded.")]
    MetadataQrDecode,

    #[error("Received metadata QR payload has unexpected structure. Can not find compact to cut payload into parts.")]
    MetadataQrUnexpectedStructure,

    #[error("No metadata entries for genesis hash {} in the database.", hex::encode(.0))]
    NoMetadata(H256),

    #[error("Metadata from scanned QR does not start with expected b`META` prefix.")]
    NoMetaPrefixQr,

    #[error("Received QR payload does not have prelude corresponding to metadata.")]
    NotMetadataQr,

    #[error("Received QR payload is not a Substrate one.")]
    NotSubstrate,

    #[error("Received QR payload is not a signable transaction.")]
    NotTransactionQr,

    #[error("Metadata in received QR payload is not V14 and is not supported.")]
    OnlyV14SupportedQr,

    #[error("Received QR payload is too short.")]
    TooShort,

    #[error("Unexpected signing algorithm.")]
    UnknownSigningAlgorithm,
}
