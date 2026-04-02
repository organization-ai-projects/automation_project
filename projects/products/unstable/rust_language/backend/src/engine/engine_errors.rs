//! projects/products/unstable/rust_language/backend/src/engine/engine_error.rs
use thiserror::Error;

use crate::app_error::AppError;

#[derive(Debug, Error)]
pub enum EngineErrors {
    #[error("Engine initialization failed: {0}")]
    Initialization(String),

    #[error("Engine runtime error: {0}")]
    Runtime(String),

    #[error("Unknown engine error: {0}")]
    Unknown(String),
}

impl From<AppError> for EngineErrors {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Io(_) => EngineErrors::Initialization(err.to_string()),
            AppError::Usage(_) => EngineErrors::Runtime(err.to_string()),
            _ => EngineErrors::Unknown(err.to_string()),
        }
    }
}
