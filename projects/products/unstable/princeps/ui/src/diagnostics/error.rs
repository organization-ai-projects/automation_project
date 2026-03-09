use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("ui error: {0}")]
    Ui(String),
}
