// projects/libraries/common_json/src/tests/json_error.rs
use crate::json_error::{JsonError, JsonErrorCode};

#[cfg(test)]
#[test]
fn test_json_error_new() {
    let error = JsonError::new(JsonErrorCode::TypeMismatch);
    assert_eq!(error.message(), "type mismatch");
}

#[test]
fn test_json_error_context() {
    let error = JsonError::new(JsonErrorCode::TypeMismatch).context("Additional context");
    assert!(error.to_string().contains("Additional context"));
}
