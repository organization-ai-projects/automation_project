// projects/libraries/symbolic/src/tests/test_helpers.rs
//! Shared test utilities and helpers to reduce boilerplate across test modules.

use crate::validation_result::ValidationResult;
use crate::validator::CodeValidator;

/// Standard test result type for consistent error handling across test modules.
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// Creates a CodeValidator instance, panicking with a helpful message if creation fails.
///
/// Use this helper to reduce boilerplate in test setup.
pub fn create_validator() -> CodeValidator {
    CodeValidator::new().expect("Failed to create CodeValidator")
}

/// Creates a CodeValidator with strict mode enabled.
pub fn create_strict_validator() -> CodeValidator {
    create_validator().with_strict_mode(true)
}

/// Asserts that a validation result contains a warning matching a substring.
///
/// This is more flexible than exact string matching and less brittle to message changes.
///
/// # Examples
/// ```ignore
/// assert_warn_contains(&validation, "unused");
/// assert_warn_contains(&validation, "Type mismatch");
/// ```
pub fn assert_warn_contains(validation: &ValidationResult, substring: &str) {
    let found = validation.warnings.iter().any(|w| w.contains(substring));

    assert!(
        found,
        "Expected warning containing '{}', but found warnings: {:?}",
        substring, validation.warnings
    );
}

/// Asserts that a validation result does NOT contain a warning matching a substring.
pub fn assert_warn_not_contains(validation: &ValidationResult, substring: &str) {
    // For variable names, check if they appear in quotes to avoid partial matches
    let pattern = if substring.chars().all(|c| c.is_alphanumeric() || c == '_') {
        format!("'{}'", substring)
    } else {
        substring.to_string()
    };

    let found = validation.warnings.iter().any(|w| w.contains(&pattern));

    assert!(
        !found,
        "Expected no warning containing '{}', but found: {:?}",
        pattern,
        validation
            .warnings
            .iter()
            .filter(|w| w.contains(&pattern))
            .collect::<Vec<_>>()
    );
}

/// Asserts that a validation result contains a warning matching ALL the given substrings.
///
/// This allows checking for multiple required parts of a warning without exact string matching.
///
/// # Examples
/// ```ignore
/// assert_warn_contains_all(&validation, &["unused_var", "never used"]);
/// assert_warn_contains_all(&validation, &["Type mismatch", "'x'"]);
/// ```
pub fn assert_warn_contains_all(validation: &ValidationResult, substrings: &[&str]) {
    let found = validation
        .warnings
        .iter()
        .any(|w| substrings.iter().all(|s| w.contains(s)));

    assert!(
        found,
        "Expected warning containing all of {:?}, but found warnings: {:?}",
        substrings, validation.warnings
    );
}

/// Asserts that a validation result has at least the minimum expected warning count.
pub fn assert_min_warnings(validation: &ValidationResult, min_count: usize) {
    assert!(
        validation.warnings.len() >= min_count,
        "Expected at least {} warnings, but found {}: {:?}",
        min_count,
        validation.warnings.len(),
        validation.warnings
    );
}

/// Asserts that a validation result is valid with no errors.
pub fn assert_valid(validation: &ValidationResult) {
    assert!(
        validation.is_valid,
        "Expected validation to be valid, but found errors: {:?}",
        validation.errors
    );
}

/// Asserts that a validation result is invalid with at least one error.
pub fn assert_invalid(validation: &ValidationResult) {
    assert!(
        !validation.is_valid,
        "Expected validation to be invalid, but it was valid"
    );
    assert!(
        !validation.errors.is_empty(),
        "Expected validation to have errors, but errors list was empty"
    );
}

/// Asserts that a validation result contains an error matching a substring.
pub fn assert_error_contains(validation: &ValidationResult, substring: &str) {
    let found = validation.errors.iter().any(|e| e.contains(substring));

    assert!(
        found,
        "Expected error containing '{}', but found errors: {:?}",
        substring, validation.errors
    );
}
