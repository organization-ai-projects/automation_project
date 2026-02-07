// projects/libraries/common_json/src/tests/json_number.rs
use crate::JsonNumber;

#[test]
fn test_json_number_from_f64() {
    let num = JsonNumber::from_f64(42.0).expect("Error creating JsonNumber: value is None");
    assert_eq!(num.as_f64(), 42.0);
}

#[test]
fn test_json_number_is_non_zero() {
    let num = JsonNumber::from_f64(0.0).expect("Error creating JsonNumber: value is None");
    assert!(!num.is_non_zero());
}
