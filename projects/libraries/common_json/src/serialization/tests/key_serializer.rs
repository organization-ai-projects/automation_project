// projects/libraries/common_json/src/serialization/tests/key_serializer.rs
use crate::json_error::JsonError;
use crate::json_error_code::JsonErrorCode;
#[cfg(test)]
use crate::serialization::key_serializer::KeySerializer;
use serde::ser::Serializer;

#[test]
fn test_serialize_bool() {
    match KeySerializer.serialize_bool(true) {
        Ok(result) => assert_eq!(result, "true"),
        Err(_) => panic!("Expected Ok, got Err"),
    }

    match KeySerializer.serialize_bool(false) {
        Ok(result) => assert_eq!(result, "false"),
        Err(_) => panic!("Expected Ok, got Err"),
    }
}

#[test]
fn test_serialize_str() {
    match KeySerializer.serialize_str("test_key") {
        Ok(result) => assert_eq!(result, "test_key"),
        Err(_) => panic!("Expected Ok, got Err"),
    }

    match KeySerializer.serialize_str("") {
        Ok(result) => assert_eq!(result, ""),
        Err(_) => panic!("Expected Ok, got Err"),
    }
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
