// projects/libraries/layers/domain/neural/src/network/network_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("Invalid layer configuration: {0}")]
    InvalidConfig(String),
}
