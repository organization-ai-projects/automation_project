// projects/products/unstable/hospital_tycoon/ui/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(String),
    #[error("ipc error: {0}")]
    Ipc(String),
    #[error("process error: {0}")]
    Process(String),
    #[error("replay error: {0}")]
    Replay(String),
}
