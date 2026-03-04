use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("transport error: {0}")]
    Transport(String),
}
