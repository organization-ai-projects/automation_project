//! projects/products/unstable/rust_language/backend/src/ai_assist/ai_errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AiErrors {
    #[error("AI improvement failed: {0}")]
    Improvement(String),

    #[error("AI validation failed: {0}")]
    Validation(String),
}
