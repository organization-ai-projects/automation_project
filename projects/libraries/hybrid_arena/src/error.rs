//! Errors that can occur when working with arenas.

use std::fmt;

/// Errors that can occur when working with arenas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArenaError {
    /// The arena would exceed 2^32 items.
    Overflow,
    /// An ID was invalid (wrong generation or out of bounds).
    InvalidId,
}

impl fmt::Display for ArenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArenaError::Overflow => {
                write!(f, "arena overflow: exceeded maximum capacity (2^32 items)")
            }
            ArenaError::InvalidId => write!(f, "invalid arena ID"),
        }
    }
}

impl std::error::Error for ArenaError {}
