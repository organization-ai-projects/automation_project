// projects/products/unstable/hospital_tycoon/backend/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(String),
    #[error("config error: {0}")]
    Config(String),
    #[error("protocol error: {0}")]
    Protocol(String),
    #[error("replay error: {0}")]
    Replay(String),
}
