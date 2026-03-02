use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Scan error: {0}")]
    Scan(String),
    #[error("Canon error: {0}")]
    Canon(String),
    #[error("Stability error: {0}")]
    Stability(String),
}
