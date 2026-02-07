// projects/libraries/common_json/src/serialization/tests/helpers.rs
use crate::Json;
use crate::serialization::helpers::{json_to_string, serialize_to_json};
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_serialize_to_json() -> TestResult {
    let value = Json::Object(
        [("key".to_string(), Json::String("value".to_string()))]
            .iter()
            .cloned()
            .collect(),
    );
    let result = serialize_to_json(&value)?;

    let Json::Object(map) = result else {
        panic!("Expected Json::Object");
    };
    assert_eq!(map.get("key"), Some(&Json::String("value".to_string())));
    Ok(())
}

#[test]
fn test_json_to_string_pretty() -> TestResult {
    let json = Json::Object(
        [
            ("key1".to_string(), Json::String("value1".to_string())),
            ("key2".to_string(), Json::String("value2".to_string())),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    let result = json_to_string(&json, true)?;
    let expected_variants = [
        "{\n  \"key1\": \"value1\",\n  \"key2\": \"value2\"\n}",
        "{\n  \"key2\": \"value2\",\n  \"key1\": \"value1\"\n}",
    ];
    assert!(expected_variants.contains(&result.as_str()));
    Ok(())
}

#[test]
fn test_json_to_string_compact() -> TestResult {
    let json = Json::Object(
        [
            ("key1".to_string(), Json::String("value1".to_string())),
            ("key2".to_string(), Json::String("value2".to_string())),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    let result = json_to_string(&json, false)?;
    let expected_variants = [
        "{\"key1\":\"value1\",\"key2\":\"value2\"}",
        "{\"key2\":\"value2\",\"key1\":\"value1\"}",
    ];
    assert!(expected_variants.contains(&result.as_str()));
    Ok(())
}
