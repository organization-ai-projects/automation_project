use thiserror::Error;

#[derive(Debug, Error)]
pub enum LifeSimError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Replay error: {0}")]
    #[allow(dead_code)]
    Replay(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("No such command")]
    NoSuchCommand,
}
