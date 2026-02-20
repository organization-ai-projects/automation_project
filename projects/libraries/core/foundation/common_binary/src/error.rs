use std::fmt;

/// Error types for binary persistence operations.
#[derive(Debug, thiserror::Error)]
pub enum BinaryError {
    /// I/O error during read or write operations
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Data corruption detected
    #[error("Corrupt data: {0}")]
    Corrupt(&'static str),

    /// Incompatible format or schema
    #[error("Incompatible: {0}")]
    Incompatible(&'static str),

    /// Encoding error
    #[error("Encode error: {0}")]
    Encode(&'static str),

    /// Decoding error
    #[error("Decode error: {0}")]
    Decode(&'static str),
}

impl serde::ser::Error for BinaryError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        BinaryError::Encode(Box::leak(msg.to_string().into_boxed_str()))
    }
}

impl serde::de::Error for BinaryError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        BinaryError::Decode(Box::leak(msg.to_string().into_boxed_str()))
    }
}
