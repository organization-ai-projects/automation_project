//! projects/products/unstable/rust_language/backend/src/cli/cli_errors.rs
use ai::AiError;
use std::{convert::From, io};
use thiserror::Error;

use crate::engine::EngineErrors;

#[derive(Debug, Error)]
pub enum CliErrors {
    #[error("Invalid CLI usage: {0}")]
    InvalidCli(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Compilation error: {0}")]
    Compilation(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Code improvement error: {0}")]
    CodeImprovement(String),

    #[error("RHL code improvement error: {0}")]
    RhlImprovement(String),

    #[error("Rust code optimization error: {0}")]
    RustOptimization(String),
}

impl From<io::Error> for CliErrors {
    fn from(error: io::Error) -> Self {
        CliErrors::Io(error.to_string())
    }
}
