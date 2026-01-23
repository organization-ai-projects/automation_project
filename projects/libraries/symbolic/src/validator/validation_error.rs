// projects/libraries/symbolic/src/validator/validation_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },
    #[error("Invalid structure: {0}")]
    InvalidStructure(String),
    #[error("Missing required element: {0}")]
    MissingElement(String),
}
