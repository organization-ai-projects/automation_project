// projects/libraries/common_json/src/tests/value.rs
use super::test_helpers::TestResult;
use crate::value::{array, null, object};
use crate::{Json, boolean, number_f64, number_i64, number_u64, string};
use serde::Serialize;

#[test]
fn test_value_object() {
    let value = object();
    assert!(value.is_object());
}

#[test]
fn test_value_array() {
    let value = array();
    assert!(value.is_array());
}

#[test]
fn test_value_null() {
    let value = null();
    assert!(value.is_null());
}

#[test]
fn test_value_boolean() {
    let value = boolean(true);
    let bool_val = value.as_bool().expect("Error extracting boolean value");
    assert!(bool_val);
}

#[test]
fn test_value_string() {
    let value = string("hello");
    let str_val = value.as_str().expect("Error extracting string value");
    assert_eq!(str_val, "hello");
}

#[test]
fn test_value_number_i64() {
    let value = number_i64(42);
    let num = value.as_i64().expect("Error extracting i64 value");
    assert_eq!(num, 42);
}

#[test]
fn test_value_number_u64() {
    let value = number_u64(42);
    let num = value.as_u64().expect("Error extracting u64 value");
    assert_eq!(num, 42);
}

#[test]
fn test_value_number_f64() {
    let json = number_f64(42.5).expect("Error creating JsonNumber: value is None");
    let num = json.as_f64().expect("Error extracting f64 value");
    assert_eq!(num, 42.5);
}

#[test]
fn test_value_from_serialize() -> TestResult {
    #[derive(Serialize)]
    struct TestStruct {
        field: String,
    }

    let test_value = TestStruct {
        field: "value".to_string(),
    };

    let json_obj = Json::from_serialize(&test_value)?;
    assert!(json_obj.is_object());
    Ok(())
}

#[test]
fn test_value_is_non_zero() {
    let value = number_i64(42);
    assert!(value.is_non_zero());

    let zero_value = number_i64(0);
    assert!(!zero_value.is_non_zero());
}

#[test]
fn test_value_from_char() {
    let value = Json::from('a');
    let char_val = value.as_str().expect("Error extracting char value");
    assert_eq!(char_val, "a");
}

#[test]
fn test_value_from_option() {
    let some_value: Json = Some(42).into();
    let num = some_value
        .as_i64()
        .expect("Error extracting i64 value from Some");
    assert_eq!(num, 42);

    let none_value: Json = None::<i32>.into();
    assert!(none_value.is_null());
}
