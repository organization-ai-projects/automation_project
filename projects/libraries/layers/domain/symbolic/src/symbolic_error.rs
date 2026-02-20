// projects/libraries/layers/domain/symbolic/src/symbolic_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SymbolicError {
    #[error("Analysis error: {0}")]
    AnalysisError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Generation error: {0}")]
    GenerationError(String),
}
