// projects/libraries/common_json/src/tests/json_number_visitor.rs
use crate::json_number::JsonNumber;
use crate::json_number_visitor::JsonNumberVisitor;
use serde::de::Visitor;
use serde::de::value::Error as SerdeError;

#[test]
fn test_visit_i64() {
    let visitor = JsonNumberVisitor;
    let result = visitor
        .visit_i64::<SerdeError>(42i64)
        .expect("visit_i64 should succeed");
    assert_eq!(result, JsonNumber::from(42i64));
}

#[test]
fn test_visit_u64() {
    let visitor = JsonNumberVisitor;
    let result = visitor
        .visit_u64::<SerdeError>(42u64)
        .expect("visit_u64 should succeed");
    assert_eq!(result, JsonNumber::from(42u64));
}

#[test]
fn test_visit_f64_valid() {
    let visitor = JsonNumberVisitor;
    let result = visitor
        .visit_f64::<SerdeError>(42.0)
        .expect("visit_f64 should succeed for finite value");
    let expected = JsonNumber::from_f64(42.0).expect("Valid value");
    assert_eq!(result, expected);
}

#[test]
fn test_visit_f64_invalid() {
    let visitor = JsonNumberVisitor;
    let result = visitor.visit_f64::<SerdeError>(f64::INFINITY);
    assert!(result.is_err());
}
