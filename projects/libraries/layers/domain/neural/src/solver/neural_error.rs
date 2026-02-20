// projects/libraries/layers/domain/neural/src/solver/neural_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeuralError {
    #[error("Generation error: {0}")]
    GenerationError(String),
    #[error("Training error: {0}")]
    TrainingError(String),
    #[error("Model not loaded")]
    ModelNotLoaded,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Save error: {0}")]
    SaveError(String),
}
