use thiserror::Error;

#[derive(Debug, Error)]
pub enum CityBuilderError {
    #[error("Invalid scenario: {0}")]
    InvalidScenario(String),
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
    #[error("Replay mismatch: {0}")]
    ReplayMismatch(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
