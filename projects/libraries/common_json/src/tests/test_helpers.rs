// projects/libraries/common_json/src/tests/test_helpers.rs
//! Shared test helpers and utilities for common_json tests.

use crate::Json;
use std::error::Error;

/// Standard test result type alias for test functions that return Result
pub type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Assert that a Json value is an object, returning a helpful error message if not
#[track_caller]
pub fn assert_json_object(json: &Json) {
    assert!(json.is_object(), "Expected Json::Object but got: {:?}", json);
}

/// Assert that a Json value is an array, returning a helpful error message if not
#[track_caller]
pub fn assert_json_array(json: &Json) {
    assert!(json.is_array(), "Expected Json::Array but got: {:?}", json);
}

/// Assert that a Json value is a number, returning a helpful error message if not
#[track_caller]
pub fn assert_json_number(json: &Json) {
    assert!(matches!(json, Json::Number(_)), "Expected Json::Number but got: {:?}", json);
}

/// Assert that a Json value is a string, returning a helpful error message if not
#[track_caller]
pub fn assert_json_string(json: &Json) {
    assert!(matches!(json, Json::String(_)), "Expected Json::String but got: {:?}", json);
}

/// Assert that a Json value is a boolean, returning a helpful error message if not
#[track_caller]
pub fn assert_json_bool(json: &Json) {
    assert!(matches!(json, Json::Bool(_)), "Expected Json::Bool but got: {:?}", json);
}

/// Assert that a Json value is null, returning a helpful error message if not
#[track_caller]
pub fn assert_json_null(json: &Json) {
    assert!(json.is_null(), "Expected Json::Null but got: {:?}", json);
}
