// projects/products/unstable/code_forge_engine/ui/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("ipc error: {0}")]
    Ipc(String),
    #[error("state error: {0}")]
    State(String),
}
