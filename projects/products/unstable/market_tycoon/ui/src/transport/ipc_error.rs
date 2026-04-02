//! projects/products/unstable/market_tycoon/ui/src/transport/ipc_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("process error: {0}")]
    Process(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("protocol error: {0}")]
    Protocol(String),
}
