use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum FuzzHarnessError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
    #[error("Unknown target: {0}")]
    UnknownTarget(String),
    #[error("Replay mismatch: {0}")]
    ReplayMismatch(String),
}
