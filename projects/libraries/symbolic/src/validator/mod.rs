// projects/libraries/symbolic/src/validator/mod.rs
// Module for code validation
pub mod code_validator;
mod dead_code_visitor;
mod import_visitor;
pub mod semantic_analyzer;
pub mod semantic_issue;
mod type_visitor;
pub mod validation_error;
pub mod validation_result;
mod variable_collector;
mod variable_visitor;

pub use code_validator::CodeValidator;
pub use semantic_analyzer::SemanticAnalyzer;
pub use semantic_issue::{SemanticIssue, SemanticIssueType, Severity};
pub use validation_error::ValidationError;
pub use validation_result::ValidationResult;

#[cfg(test)]
mod tests;
