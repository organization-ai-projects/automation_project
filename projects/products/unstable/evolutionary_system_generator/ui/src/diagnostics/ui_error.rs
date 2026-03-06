// projects/products/unstable/evolutionary_system_generator/ui/src/diagnostics/ui_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("No backend connected")]
    NoBackend,
}
