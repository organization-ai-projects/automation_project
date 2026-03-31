#[derive(Debug, thiserror::Error)]
pub enum TacticsGridError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] common_json::JsonError),
    #[error("invalid scenario: {0}")]
    InvalidScenario(String),
    #[error("battle: {0}")]
    Battle(String),
    #[error("replay mismatch: {0}")]
    ReplayMismatch(String),
}
