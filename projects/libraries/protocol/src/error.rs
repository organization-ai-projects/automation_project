// projects/libraries/protocol/src/error.rs
use std::fmt;

/// Errors that can occur during protocol validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Command or event name is empty or contains only whitespace
    EmptyName,
    /// Payload or data is empty or contains only whitespace
    EmptyPayload,
    /// Name contains invalid characters
    InvalidNameFormat(String),
    /// Payload exceeds maximum allowed size
    PayloadTooLarge { size: usize, max: usize },
    /// Name exceeds maximum allowed length
    NameTooLong { length: usize, max: usize },
    /// Timestamp is invalid (e.g., in the future beyond acceptable threshold)
    InvalidTimestamp(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "Name cannot be empty"),
            ValidationError::EmptyPayload => write!(f, "Payload/data cannot be empty"),
            ValidationError::InvalidNameFormat(name) => {
                write!(f, "Name '{}' contains invalid characters", name)
            }
            ValidationError::PayloadTooLarge { size, max } => {
                write!(f, "Payload size {} bytes exceeds maximum of {} bytes", size, max)
            }
            ValidationError::NameTooLong { length, max } => {
                write!(f, "Name length {} exceeds maximum of {}", length, max)
            }
            ValidationError::InvalidTimestamp(reason) => {
                write!(f, "Invalid timestamp: {}", reason)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
