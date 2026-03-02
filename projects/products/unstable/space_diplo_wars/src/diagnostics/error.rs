use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpaceDiploWarsError {
    #[error("invalid CLI arguments: {0}")]
    InvalidCli(String),
    #[error("invalid scenario or config: {0}")]
    InvalidScenario(String),
    #[error("invalid orders: {0}")]
    InvalidOrders(String),
    #[error("replay mismatch: {0}")]
    ReplayMismatch(String),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
