// projects/products/unstable/protocol_builder/tooling/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolingError {
    #[error("validation failed: {0}")]
    ValidationFailed(String),

    #[error("missing argument: {0}")]
    MissingArgument(&'static str),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
