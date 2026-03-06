// projects/products/unstable/meta_determinism_guard/ui/src/diagnostics/ui_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] common_json::JsonError),
    #[error("Invalid usage: {0}")]
    InvalidUsage(String),
    #[error("Transport error: {0}")]
    Transport(String),
}
