use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid CLI: {0}")]
    InvalidCli(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("replay mismatch: {0}")]
    ReplayMismatch(String),

    #[error("dsl error: {0}")]
    DslError(String),
}
