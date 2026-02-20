// projects/libraries/layers/domain/symbolic/src/rules/rules_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RulesError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}
