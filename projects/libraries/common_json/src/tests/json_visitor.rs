// projects/libraries/common_json/src/tests/json_visitor.rs
use crate::Json;
use crate::JsonError;
use crate::json_visitor::JsonVisitor;
use serde::de::IntoDeserializer;
use serde::de::Visitor;
use serde::de::value::Error as SerdeError;
use serde::de::value::{MapDeserializer, SeqDeserializer};

#[test]
fn test_visit_unit() {
    let visitor = JsonVisitor;
    let json = visitor
        .visit_unit::<SerdeError>()
        .expect("visit_unit should succeed");
    assert_eq!(json, Json::Null);
}

#[test]
fn test_visit_bool() {
    let visitor = JsonVisitor;
    let json = visitor
        .visit_bool::<SerdeError>(true)
        .expect("visit_bool should succeed");
    assert_eq!(json, Json::Bool(true));
}

#[test]
fn test_visit_i64() {
    let visitor = JsonVisitor;
    let json = visitor
        .visit_i64::<SerdeError>(42)
        .expect("visit_i64 should succeed");
    assert!(matches!(json, Json::Number(_)));
}

#[test]
fn test_visit_u64() {
    let visitor = JsonVisitor;
    let json = visitor
        .visit_u64::<SerdeError>(42)
        .expect("visit_u64 should succeed");
    assert!(matches!(json, Json::Number(_)));
}

#[test]
fn test_visit_f64_valid() {
    let visitor = JsonVisitor;
    let json = visitor
        .visit_f64::<SerdeError>(42.0)
        .expect("visit_f64 should succeed for finite value");
    assert!(matches!(json, Json::Number(_)));
}

#[test]
fn test_visit_f64_invalid() {
    let visitor = JsonVisitor;
    let result = visitor.visit_f64::<SerdeError>(f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_visit_str() {
    let visitor = JsonVisitor;
    let json = visitor
        .visit_str::<SerdeError>("hello")
        .expect("visit_str should succeed");
    assert_eq!(json, Json::String("hello".to_string()));
}

#[test]
fn test_visit_seq() {
    let visitor = JsonVisitor;
    let seq = vec!["true", "null"];
    let deserializer = SeqDeserializer::new(
        seq.into_iter()
            .map(<&str as IntoDeserializer<'_, JsonError>>::into_deserializer),
    );
    let json = visitor
        .visit_seq(deserializer)
        .expect("visit_seq should succeed");
    assert!(matches!(json, Json::Array(_)));
}

#[test]
fn test_visit_map() {
    let visitor = JsonVisitor;
    let map = vec![("key", "true")];
    let deserializer = MapDeserializer::new(map.into_iter().map(|(k, v)| {
        (
            <&str as IntoDeserializer<'_, JsonError>>::into_deserializer(k),
            <&str as IntoDeserializer<'_, JsonError>>::into_deserializer(v),
        )
    }));
    let json = visitor
        .visit_map(deserializer)
        .expect("visit_map should succeed");
    assert!(matches!(json, Json::Object(_)));
}
