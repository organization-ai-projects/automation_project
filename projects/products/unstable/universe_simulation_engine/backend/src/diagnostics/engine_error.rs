use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("simulation error: {0}")]
    Sim(String),
    #[error("invalid config: {0}")]
    InvalidConfig(String),
    #[error("binary codec error: {0}")]
    BinaryCodec(String),
    #[error("RON codec error: {0}")]
    RonCodec(String),
}
