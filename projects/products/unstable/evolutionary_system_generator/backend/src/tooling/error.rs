use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolingError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
