// projects/libraries/common_json/src/serialization/tests/key_serializer.rs
use crate::json_error::JsonError;
use crate::json_error_code::JsonErrorCode;
use crate::serialization::key_serializer::KeySerializer;
use serde::ser::Serializer;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_serialize_bool() -> TestResult {
    let result = KeySerializer.serialize_bool(true)?;
    assert_eq!(result, "true");

    let result = KeySerializer.serialize_bool(false)?;
    assert_eq!(result, "false");
    Ok(())
}

#[test]
fn test_serialize_str() -> TestResult {
    let result = KeySerializer.serialize_str("test_key")?;
    assert_eq!(result, "test_key");

    let result = KeySerializer.serialize_str("")?;
    assert_eq!(result, "");
    Ok(())
}

#[test]
fn test_serialize_unsupported_type() {
    let result = KeySerializer.serialize_bytes(b"unsupported");
    assert!(matches!(
        result,
        Err(JsonError {
            code: JsonErrorCode::Custom,
            ..
        })
    ));
}
