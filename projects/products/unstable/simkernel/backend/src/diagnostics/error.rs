use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum SimError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid scenario: {0}")]
    InvalidScenario(String),
    #[error("Replay mismatch: {0}")]
    ReplayMismatch(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
