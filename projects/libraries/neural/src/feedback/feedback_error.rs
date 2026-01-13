// projects/libraries/neural/src/feedback/feedback_error.rs
use crate::network::neural_net::NetworkError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedbackError {
    #[error("Invalid feedback format: {0}")]
    InvalidFormat(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),
    #[error("Training error: {0}")]
    TrainingError(String),
    #[error("Insufficient feedback for adjustment")]
    InsufficientFeedback,
}
