// projects/libraries/layers/domain/neural/src/tokenization/tokenization_error.rs
use common_json::JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizationError {
    #[error("Unknown token: {0}")]
    UnknownToken(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(JsonError),
}

impl From<JsonError> for TokenizationError {
    fn from(err: JsonError) -> Self {
        TokenizationError::SerializationError(err)
    }
}
