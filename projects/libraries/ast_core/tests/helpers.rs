// projects/libraries/ast_core/tests/helpers.rs
//! Shared test helpers for integration tests

use ast_core::{AstErrorKind, AstNode, AstValidationError};

/// Helper to assert that a validation error matches a specific pattern
pub fn assert_error_matches(result: Result<(), AstValidationError>, pattern: &str) {
    assert!(result.is_err(), "Expected an error but got Ok");
    let err = result.unwrap_err();
    let error_msg = format!("{}", err);
    assert!(
        error_msg.contains(pattern),
        "Error message '{}' does not contain '{}'",
        error_msg,
        pattern
    );
}

/// Helper to assert that a validation error has a specific kind using matches!
pub fn assert_error_kind_matches(
    result: Result<(), AstValidationError>,
    check: impl Fn(&AstErrorKind) -> bool,
    desc: &str,
) {
    assert!(result.is_err(), "Expected an error but got Ok");
    let err = result.unwrap_err();
    assert!(check(&err.kind), "Error kind check failed: {}", desc);
}

/// Helper to assert string key value
pub fn assert_string_key(node: &AstNode, key: &str, expected: &str) {
    let value = node
        .get(key)
        .unwrap_or_else(|| panic!("Missing key '{}'", key));
    assert_eq!(
        value.as_string(),
        Some(expected),
        "Key '{}' has unexpected value",
        key
    );
}

/// Helper to assert integer key value
pub fn assert_int_key(node: &AstNode, key: &str, expected: i64) {
    let value = node
        .get(key)
        .unwrap_or_else(|| panic!("Missing key '{}'", key));
    assert_eq!(
        value.as_number().and_then(|n| n.as_i64()),
        Some(expected),
        "Key '{}' has unexpected value",
        key
    );
}

/// Helper to assert boolean key value
pub fn assert_bool_key(node: &AstNode, key: &str, expected: bool) {
    let value = node
        .get(key)
        .unwrap_or_else(|| panic!("Missing key '{}'", key));
    assert_eq!(
        value.as_bool(),
        Some(expected),
        "Key '{}' has unexpected value",
        key
    );
}

/// Helper to assert that a node has a nested key path with expected string value
pub fn assert_nested_string(node: &AstNode, keys: &[&str], expected: &str) {
    let mut current = node;
    for (i, key) in keys.iter().enumerate() {
        current = current
            .get(key)
            .unwrap_or_else(|| panic!("Missing key '{}' at depth {}", key, i));
    }
    assert_eq!(
        current.as_string(),
        Some(expected),
        "Nested key path {:?} has unexpected value",
        keys
    );
}
