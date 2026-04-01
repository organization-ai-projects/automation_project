use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid CLI: {0}")]
    InvalidCli(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("query error: {0}")]
    Query(String),
    #[error("policy error: {0}")]
    Policy(String),
}
