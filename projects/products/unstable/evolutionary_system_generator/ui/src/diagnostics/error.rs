use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("No backend connected")]
    NoBackend,
}
