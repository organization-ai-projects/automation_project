use thiserror::Error;
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ToolingError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Contract violation: {0}")]
    ContractViolation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
