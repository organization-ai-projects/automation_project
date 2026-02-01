// projects/libraries/common_json/src/tests/value.rs
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
    match value.as_bool() {
        Some(bool_val) => assert!(bool_val),
        None => panic!("Error extracting boolean value"),
    }
}

#[test]
fn test_value_string() {
    let value = string("hello");
    match value.as_str() {
        Some(str_val) => assert_eq!(str_val, "hello"),
        None => panic!("Error extracting string value"),
    }
}

#[test]
fn test_value_number_i64() {
    let value = number_i64(42);
    match value.as_i64() {
        Some(num) => assert_eq!(num, 42),
        None => panic!("Error extracting i64 value"),
    }
}

#[test]
fn test_value_number_u64() {
    let value = number_u64(42);
    match value.as_u64() {
        Some(num) => assert_eq!(num, 42),
        None => panic!("Error extracting u64 value"),
    }
}

#[test]
fn test_value_number_f64() {
    let value = number_f64(42.5);
    match value {
        Some(json) => match json.as_f64() {
            Some(num) => assert_eq!(num, 42.5),
            None => panic!("Error extracting f64 value"),
        },
        None => panic!("Error creating JsonNumber: value is None"),
    }
}

#[test]
fn test_value_from_serialize() {
    #[derive(Serialize)]
    struct TestStruct {
        field: String,
    }

    let test_value = TestStruct {
        field: "value".to_string(),
    };

    let json = Json::from_serialize(&test_value);
    match json {
        Ok(json_obj) => assert!(json_obj.is_object()),
        Err(err) => panic!("Error serializing value: {:?}", err),
    }
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
    match value.as_str() {
        Some(char_val) => assert_eq!(char_val, "a"),
        None => panic!("Error extracting char value"),
    }
}

#[test]
fn test_value_from_option() {
    let some_value: Json = Some(42).into();
    match some_value.as_i64() {
        Some(num) => assert_eq!(num, 42),
        None => panic!("Error extracting i64 value from Some"),
    }

    let none_value: Json = None::<i32>.into();
    assert!(none_value.is_null());
}
