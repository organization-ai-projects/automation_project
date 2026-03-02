use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Transport error: {0}")]
    Transport(String),
}
