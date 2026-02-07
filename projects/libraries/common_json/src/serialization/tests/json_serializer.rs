// projects/libraries/common_json/src/serialization/tests/json_serializer.rs
use crate::Json;
use crate::JsonNumber;
use crate::serialization::json_serializer::JsonSerializer;
use crate::tests::test_helpers::TestResult;
use serde::ser::Serializer;

#[test]
fn test_serialize_bool() -> TestResult {
    let result = JsonSerializer.serialize_bool(true)?;
    assert_eq!(result, Json::Bool(true));

    let result = JsonSerializer.serialize_bool(false)?;
    assert_eq!(result, Json::Bool(false));
    Ok(())
}

#[test]
fn test_serialize_str() -> TestResult {
    let result = JsonSerializer.serialize_str("test")?;
    assert_eq!(result, Json::String("test".to_string()));
    Ok(())
}

#[test]
fn test_serialize_number() -> TestResult {
    let result = JsonSerializer.serialize_i64(42_i64)?;
    assert_eq!(result, Json::Number(JsonNumber::from(42_i64)));

    let result = JsonSerializer.serialize_f64(std::f64::consts::PI)?;
    assert_eq!(
        result,
        Json::Number(JsonNumber::from_f64(std::f64::consts::PI).expect("valid f64 in test"))
    );
    Ok(())
}

#[test]
fn test_serialize_none() -> TestResult {
    let result = JsonSerializer.serialize_none()?;
    assert_eq!(result, Json::Null);
    Ok(())
}

#[test]
fn test_serialize_some() -> TestResult {
    let result = JsonSerializer.serialize_some(&"value")?;
    assert_eq!(result, Json::String("value".to_string()));
    Ok(())
}
