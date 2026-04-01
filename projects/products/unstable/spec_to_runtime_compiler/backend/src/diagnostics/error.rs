use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("parse error: {0}")]
    Parse(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("invariant violation: {0}")]
    Invariant(String),

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
        let err = CompilerError::Parse("unexpected token".to_string());
        assert_eq!(err.to_string(), "parse error: unexpected token");
    }

    #[test]
    fn determinism_error_display() {
        let err = CompilerError::Determinism("field uses Instant".to_string());
        assert_eq!(err.to_string(), "determinism violation: field uses Instant");
    }
}
