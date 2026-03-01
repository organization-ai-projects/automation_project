use thiserror::Error;

#[derive(Debug, Error)]
pub enum ColonyManagerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Simulation error: {0}")]
    Sim(String),
    #[error("Replay mismatch: {0}")]
    ReplayMismatch(String),
    #[error("Invalid scenario: {0}")]
    InvalidScenario(String),
}
