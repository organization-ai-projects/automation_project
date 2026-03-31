use thiserror::Error;

/// Backend error types.
#[derive(Debug, Error)]
pub enum Error {
    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("plugin not found: {0}")]
    PluginNotFound(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("io error: {0}")]
    Io(String),
}
