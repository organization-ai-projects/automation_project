// projects/libraries/common_json/src/tests/json_visitor.rs
use super::test_helpers::{TestResult, assert_json_number};
use crate::Json;
use crate::JsonError;
use crate::json_visitor::JsonVisitor;
use serde::de::IntoDeserializer;
use serde::de::Visitor;
use serde::de::value::Error as SerdeError;
use serde::de::value::{MapDeserializer, SeqDeserializer};

#[test]
fn test_visit_unit() -> TestResult {
    let visitor = JsonVisitor;
    let json = visitor.visit_unit::<SerdeError>()?;
    assert_eq!(json, Json::Null);
    Ok(())
}

#[test]
fn test_visit_bool() -> TestResult {
    let visitor = JsonVisitor;
    let json = visitor.visit_bool::<SerdeError>(true)?;
    assert_eq!(json, Json::Bool(true));
    Ok(())
}

#[test]
fn test_visit_i64() -> TestResult {
    let visitor = JsonVisitor;
    let json = visitor.visit_i64::<SerdeError>(42)?;
    assert_json_number(&json);
    Ok(())
}

#[test]
fn test_visit_u64() -> TestResult {
    let visitor = JsonVisitor;
    let json = visitor.visit_u64::<SerdeError>(42)?;
    assert_json_number(&json);
    Ok(())
}

#[test]
fn test_visit_f64_valid() -> TestResult {
    let visitor = JsonVisitor;
    let json = visitor.visit_f64::<SerdeError>(42.0)?;
    assert_json_number(&json);
    Ok(())
}

#[test]
fn test_visit_f64_invalid() {
    let visitor = JsonVisitor;
    let result = visitor.visit_f64::<SerdeError>(f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_visit_str() -> TestResult {
    let visitor = JsonVisitor;
    let json = visitor.visit_str::<SerdeError>("hello")?;
    assert_eq!(json, Json::String("hello".to_string()));
    Ok(())
}

#[test]
fn test_visit_seq() -> TestResult {
    let visitor = JsonVisitor;
    let seq = vec!["true", "null"];
    let deserializer = SeqDeserializer::new(
        seq.into_iter()
            .map(<&str as IntoDeserializer<'_, JsonError>>::into_deserializer),
    );
    let json = visitor.visit_seq(deserializer)?;
    assert!(matches!(json, Json::Array(_)));
    Ok(())
}

#[test]
fn test_visit_map() -> TestResult {
    let visitor = JsonVisitor;
    let map = vec![("key", "true")];
    let deserializer = MapDeserializer::new(map.into_iter().map(|(k, v)| {
        (
            <&str as IntoDeserializer<'_, JsonError>>::into_deserializer(k),
            <&str as IntoDeserializer<'_, JsonError>>::into_deserializer(v),
        )
    }));
    let json = visitor.visit_map(deserializer)?;
    assert!(matches!(json, Json::Object(_)));
    Ok(())
}
