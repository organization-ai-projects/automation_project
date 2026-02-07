// projects/libraries/ast_core/src/tests/test_helpers.rs
//! Test helper functions for ast_core tests

#![allow(dead_code)]

use crate::{AstBuilder, AstErrorKind, AstNode, AstValidationError};

/// Helper to assert that a node has a specific key with an expected value
///
/// # Example
/// ```ignore
/// assert_key_eq(&node, "name", |n| n.as_string().map(|s| s.to_string()), Some("test".to_string()));
/// ```
pub(crate) fn assert_key_eq<T, F>(node: &AstNode, key: &str, extractor: F, expected: T)
where
    T: PartialEq + std::fmt::Debug,
    F: FnOnce(&AstNode) -> T,
{
    let value = node
        .get(key)
        .unwrap_or_else(|| panic!("Missing key '{}'", key));
    let actual = extractor(value);
    assert_eq!(actual, expected, "Key '{}' has unexpected value", key);
}

/// Helper to assert that a validation error has a specific kind
pub(crate) fn assert_error_kind(
    result: Result<(), AstValidationError>,
    expected_kind: fn(&AstErrorKind) -> bool,
) {
    assert!(result.is_err(), "Expected an error but got Ok");
    let err = result.unwrap_err();
    assert!(
        expected_kind(&err.kind),
        "Error kind mismatch: {:?}",
        err.kind
    );
}

/// Helper to assert that a validation error matches a specific pattern
pub(crate) fn assert_error_matches(result: Result<(), AstValidationError>, pattern: &str) {
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

/// Helper to build a deeply nested object structure for testing
pub(crate) fn build_nested_object(depth: usize) -> AstNode {
    let mut node = AstBuilder::null();
    for _ in 0..depth {
        node = AstBuilder::object(vec![("key", node)]);
    }
    node
}

/// Helper to build a wide object with many keys
pub(crate) fn build_wide_object(num_keys: usize) -> AstNode {
    let fields: Vec<_> = (0..num_keys)
        .map(|i| (format!("key{}", i), AstBuilder::int(i as i64)))
        .collect();
    AstBuilder::object(fields)
}

/// Helper to assert string key value
pub(crate) fn assert_string_key(node: &AstNode, key: &str, expected: &str) {
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
pub(crate) fn assert_int_key(node: &AstNode, key: &str, expected: i64) {
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
pub(crate) fn assert_bool_key(node: &AstNode, key: &str, expected: bool) {
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
pub(crate) fn assert_nested_string(node: &AstNode, keys: &[&str], expected: &str) {
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
