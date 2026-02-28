use crate::error::EngineError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),
    #[error("Unknown command")]
    NoSuchCommand,
}
