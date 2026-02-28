use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("state error: {0}")]
    State(String),
}
