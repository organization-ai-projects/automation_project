// projects/libraries/neural/src/generation/generation_error.rs
use thiserror::Error;

use crate::network::NetworkError;

#[derive(Debug, Error)]
#[error("{0}")]
pub enum GenerationError {
    #[error("Model not loaded")]
    ModelNotLoaded,

    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    #[error("Generation failed: {0}")]
    GenerationFailed(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),
}
