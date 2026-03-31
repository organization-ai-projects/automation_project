use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("ipc error: {0}")]
    Ipc(String),
    #[error("state error: {0}")]
    State(String),
}
