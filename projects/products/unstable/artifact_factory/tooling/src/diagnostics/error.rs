use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolingError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
}
