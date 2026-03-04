use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] common_json::JsonError),
    #[error("Validation error: {0}")]
    Validation(String),
}
