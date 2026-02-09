// projects/products/unstable/autonomous_dev_ai/src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Neural error: {0}")]
    Neural(String),

    #[error("Symbolic error: {0}")]
    Symbolic(String),

    #[error("Tool error: {0}")]
    Tool(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Objective violation: {0}")]
    ObjectiveViolation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Bincode error: {0}")]
    Bincode(String),

    #[error("Ron error: {0}")]
    Ron(String),
}

pub type AgentResult<T> = Result<T, AgentError>;
