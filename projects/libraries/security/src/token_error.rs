// projects/libraries/security/src/token_error.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenError {
    InvalidUserIdFormat,
    InvalidUserIdValue,
    InvalidDuration,
    InvalidSessionId,
    TimestampOverflow,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::InvalidUserIdFormat => write!(f, "User ID is not a valid number"),
            TokenError::InvalidUserIdValue => write!(f, "Invalid user ID provided"),
            TokenError::InvalidDuration => write!(f, "Invalid token duration"),
            TokenError::InvalidSessionId => write!(f, "Invalid session id"),
            TokenError::TimestampOverflow => write!(f, "Timestamp overflow"),
        }
    }
}

impl std::error::Error for TokenError {}
