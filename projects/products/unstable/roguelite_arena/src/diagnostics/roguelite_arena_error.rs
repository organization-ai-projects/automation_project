use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum RogueliteArenaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("Simulation error: {0}")]
    Sim(String),
    #[error("Replay mismatch: {0}")]
    ReplayMismatch(String),
    #[error("Invalid scenario: {0}")]
    InvalidScenario(String),
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
}
