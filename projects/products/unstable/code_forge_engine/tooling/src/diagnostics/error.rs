// projects/products/unstable/code_forge_engine/tooling/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolingError {
    #[error("io error: {0}")]
    Io(String),
    #[error("mismatch: {0}")]
    Mismatch(String),
}
