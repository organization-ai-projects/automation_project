// projects/products/unstable/protocol_builder/backend/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("no schema loaded")]
    NoSchemaLoaded,

    #[error("schema validation failed: {reason}")]
    SchemaInvalid { reason: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(String),
}
