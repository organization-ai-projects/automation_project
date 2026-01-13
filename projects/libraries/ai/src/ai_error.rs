// projects/libraries/ai/src/ai_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AiError {
    #[error("Symbolic error: {0}")]
    SymbolicError(#[from] symbolic::SymbolicError),
    #[error("Neural error: {0}")]
    NeuralError(#[from] neural::NeuralError),
    #[error("Task error: {0}")]
    TaskError(String),
}
