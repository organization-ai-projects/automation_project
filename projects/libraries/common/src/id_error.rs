// projects/libraries/common/src/id_error.rs
use std::fmt;

#[derive(Debug)]
pub enum IdError {
    InvalidLen,
    InvalidHex,
}

impl std::error::Error for IdError {}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::InvalidLen => write!(f, "Invalid length for ID"),
            IdError::InvalidHex => write!(f, "Invalid hex format for ID"),
        }
    }
}
