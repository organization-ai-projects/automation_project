use thiserror::Error;
#[derive(Error, Debug)]
pub enum UiError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("Backend spawn error: {0}")]
    BackendSpawn(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}
