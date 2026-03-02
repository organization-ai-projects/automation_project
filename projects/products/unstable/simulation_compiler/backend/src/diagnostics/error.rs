// projects/products/unstable/simulation_compiler/backend/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("determinism violation: {0}")]
    Determinism(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_display() {
        let e = CompilerError::Parse("unexpected token".to_string());
        assert!(e.to_string().contains("parse error"));
    }

    #[test]
    fn determinism_error_display() {
        let e = CompilerError::Determinism("wall-clock forbidden".to_string());
        assert!(e.to_string().contains("determinism violation"));
    }
}
