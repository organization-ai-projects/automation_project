// projects/products/unstable/simulation_compiler/tooling/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolingError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("golden mismatch: {0}")]
    GoldenMismatch(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("cli error: {0}")]
    Cli(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_error_display() {
        let e = ToolingError::Validation("missing file".to_string());
        assert!(e.to_string().contains("validation error"));
    }

    #[test]
    fn golden_mismatch_display() {
        let e = ToolingError::GoldenMismatch("hash differs".to_string());
        assert!(e.to_string().contains("golden mismatch"));
    }
}
