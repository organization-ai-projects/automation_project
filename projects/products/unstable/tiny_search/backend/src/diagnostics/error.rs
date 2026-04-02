use thiserror::Error;

/// Backend error types.
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("io error: {0}")]
    Io(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}
