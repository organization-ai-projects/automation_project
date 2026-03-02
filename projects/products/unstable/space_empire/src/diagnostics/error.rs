use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum SpaceEmpireError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid scenario: {0}")]
    InvalidScenario(String),
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
    #[error("Replay mismatch: {0}")]
    ReplayMismatch(String),
    #[error("Queue full: {0}")]
    QueueFull(String),
    #[error("Insufficient resources: {0}")]
    InsufficientResources(String),
}
