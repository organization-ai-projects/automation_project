use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("IO error: {0}")]
    Io(String),
}
