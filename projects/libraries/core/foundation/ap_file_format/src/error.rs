/// Error types for AP file format operations.
#[derive(Debug, thiserror::Error)]
pub enum ApFileError {
    /// I/O error during read or write operations
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Data corruption detected (e.g. checksum mismatch, truncated file)
    #[error("Corrupt data: {0}")]
    Corrupt(&'static str),

    /// Incompatible format, version, or schema
    #[error("Incompatible: {0}")]
    Incompatible(&'static str),

    /// Encoding error
    #[error("Encode error: {0}")]
    Encode(String),

    /// Decoding error
    #[error("Decode error: {0}")]
    Decode(String),

    /// Content type is not valid for the requested operation
    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    /// Invalid path for safe write
    #[error("Invalid target path: {0}")]
    InvalidPath(String),
}

pub type ApFileResult<T> = Result<T, ApFileError>;
