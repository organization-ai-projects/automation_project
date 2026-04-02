use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranspileValidatorError {
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}
