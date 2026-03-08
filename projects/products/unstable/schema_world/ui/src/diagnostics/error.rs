use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing argument: {0}")]
    MissingArgument(&'static str),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
    #[error("spawn failed: {0}")]
    SpawnFailed(String),
    #[error("ipc error: {0}")]
    IpcError(String),
}

pub type UiError = Error;
