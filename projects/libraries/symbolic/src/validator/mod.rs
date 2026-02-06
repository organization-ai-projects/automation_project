// projects/libraries/symbolic/src/validator/mod.rs
// Module for code validation
pub mod code_validator;
pub mod validation_error;
pub mod validation_result;
pub mod semantic_issue;
pub mod semantic_analyzer;

pub use code_validator::CodeValidator;
pub use validation_error::ValidationError;
pub use validation_result::ValidationResult;
pub use semantic_issue::{SemanticIssue, SemanticIssueType, Severity};
pub use semantic_analyzer::SemanticAnalyzer;

#[cfg(test)]
mod tests;
