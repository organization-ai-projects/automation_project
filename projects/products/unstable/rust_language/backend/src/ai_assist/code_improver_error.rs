//! projects/products/unstable/rust_language/backend/src/ai_assist/code_improver_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodeImproverError {
    #[error("Improvement failed: {0}")]
    ImprovementFailed(String),
}
