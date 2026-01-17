// projects/libraries/ast_core/src/ast_validation_error.rs
use std::fmt;

use crate::{AstErrorKind, AstPath};

/// A validation error with path information.
#[derive(Clone, Debug, PartialEq)]
pub struct AstValidationError {
    /// Path to the error location
    pub path: AstPath,
    /// The error kind
    pub kind: AstErrorKind,
}

impl fmt::Display for AstValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format path if not empty
        if !self.path.0.is_empty() {
            write!(f, "at {}: ", self.path)?;
        }
        match &self.kind {
            AstErrorKind::MaxDepth { max, got } => {
                write!(f, "Exceeded maximum depth: {} (got: {})", max, got)
            }
            AstErrorKind::MaxSize { kind, max } => {
                write!(f, "Exceeded maximum size for {}: {}", kind, max)
            }
            AstErrorKind::DuplicateKey { key } => write!(f, "Duplicate key found: {}", key),
        }
    }
}

impl std::error::Error for AstValidationError {}
