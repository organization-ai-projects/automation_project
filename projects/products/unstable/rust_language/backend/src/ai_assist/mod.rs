//! projects/products/unstable/rust_language/backend/src/ai_assist/mod.rs
mod ai_errors;
mod code_improver;
mod code_improver_error;
mod error_analyzer;
mod error_analyzer_error;
mod transpile_validator;
mod transpile_validator_error;

#[cfg(test)]
mod tests;

pub(crate) use ai_errors::AiErrors;
pub(crate) use code_improver::CodeImprover;
pub(crate) use code_improver_error::CodeImproverError;
pub(crate) use error_analyzer::ErrorAnalyzer;
pub(crate) use error_analyzer_error::ErrorAnalyzerError;
pub(crate) use transpile_validator::TranspileValidator;
pub(crate) use transpile_validator_error::TranspileValidatorError;
