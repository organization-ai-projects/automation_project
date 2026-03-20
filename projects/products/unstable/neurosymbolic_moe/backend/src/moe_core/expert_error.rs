use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum ExpertError {
    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("expert not available: {0}")]
    NotAvailable(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("operation timed out")]
    Timeout,

    #[error("policy violation: {0}")]
    PolicyViolation(String),
}
