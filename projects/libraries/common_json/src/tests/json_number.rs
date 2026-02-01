// projects/libraries/common_json/src/tests/json_number.rs
#[cfg(test)]
use crate::JsonNumber;
#[test]
fn test_json_number_from_f64() {
    let number = JsonNumber::from_f64(42.0);
    match number {
        Some(num) => assert_eq!(num.as_f64(), 42.0),
        None => panic!("Error creating JsonNumber: value is None"),
    }
}

#[test]
fn test_json_number_is_non_zero() {
    let number = JsonNumber::from_f64(0.0);
    match number {
        Some(num) => assert!(!num.is_non_zero()),
        None => panic!("Error creating JsonNumber: value is None"),
    }
}
