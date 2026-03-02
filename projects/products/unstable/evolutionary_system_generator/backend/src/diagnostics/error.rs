use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("No search active")]
    NoActiveSearch,
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Replay error: {0}")]
    Replay(String),
}
