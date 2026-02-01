// projects/libraries/common_json/src/tests/json_error_code.rs
use crate::json_error_code::*;

#[test]
fn test_json_error_code_equality() {
    let error1 = JsonErrorCode::Serialize;
    let error2 = JsonErrorCode::Serialize;
    let error3 = JsonErrorCode::TypeMismatch;
    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_json_error_code_debug() {
    let error = JsonErrorCode::Io;
    assert_eq!(format!("{:?}", error), "Io");
}
