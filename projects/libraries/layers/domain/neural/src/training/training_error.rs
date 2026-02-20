// projects/libraries/layers/domain/neural/src/training/training_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrainingError {
    #[error("Invalid input data: {0}")]
    InvalidInput(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}
