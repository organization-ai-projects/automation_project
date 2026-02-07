// projects/libraries/common_json/src/tests/json_number_visitor.rs
use crate::json_number::JsonNumber;
use crate::json_number_visitor::JsonNumberVisitor;
use super::test_helpers::TestResult;
use serde::de::Visitor;
use serde::de::value::Error as SerdeError;

#[test]
fn test_visit_i64() -> TestResult {
    let visitor = JsonNumberVisitor;
    let result = visitor.visit_i64::<SerdeError>(42i64)?;
    assert_eq!(result, JsonNumber::from(42i64));
    Ok(())
}

#[test]
fn test_visit_u64() -> TestResult {
    let visitor = JsonNumberVisitor;
    let result = visitor.visit_u64::<SerdeError>(42u64)?;
    assert_eq!(result, JsonNumber::from(42u64));
    Ok(())
}

#[test]
fn test_visit_f64_valid() -> TestResult {
    let visitor = JsonNumberVisitor;
    let result = visitor.visit_f64::<SerdeError>(42.0)?;
    let expected = JsonNumber::from_f64(42.0).expect("Valid value");
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_visit_f64_invalid() {
    let visitor = JsonNumberVisitor;
    let result = visitor.visit_f64::<SerdeError>(f64::INFINITY);
    assert!(result.is_err());
}
