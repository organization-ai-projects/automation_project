// projects/products/unstable/code_forge_engine/backend/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgeError {
    #[error("io error: {0}")]
    Io(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("encode error: {0}")]
    Encode(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
}
