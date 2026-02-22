// projects/products/unstable/auto_manager_ai/src/adapters/error.rs

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterErrorKind {
    Retryable,
    NonRetryable,
    Policy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterError {
    pub kind: AdapterErrorKind,
    pub code: &'static str,
    pub message: String,
}

impl AdapterError {
    pub fn retryable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: AdapterErrorKind::Retryable,
            code,
            message: message.into(),
        }
    }

    pub fn non_retryable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: AdapterErrorKind::NonRetryable,
            code,
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn policy(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: AdapterErrorKind::Policy,
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}::{:?}] {}", self.code, self.kind, self.message)
    }
}
