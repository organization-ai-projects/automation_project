// projects/products/unstable/protocol_builder/ui/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("backend process failed to spawn: {0}")]
    SpawnFailed(String),

    #[error("IPC communication error: {0}")]
    IpcError(String),

    #[error("missing argument: {0}")]
    MissingArgument(&'static str),
}
