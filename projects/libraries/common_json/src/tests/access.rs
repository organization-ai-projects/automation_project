// projects/libraries/common_json/src/tests/access.rs
use crate::Json;
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;
use std::collections::HashMap;

#[test]
fn test_get_field() {
    let json = Json::Object(HashMap::from_iter(vec![(
        "key".to_string(),
        Json::from("value"),
    )]));

    match json.get_field("key") {
        Ok(value) => assert_eq!(value, &Json::from("value")),
        Err(e) => panic!("Error accessing field: {:?}", e),
    }
    assert!(json.get_field("missing").is_err());
}

#[test]
fn test_get_index() {
    let json = Json::Array(vec![Json::from(1), Json::from(2), Json::from(3)]);

    match json.get_index(1) {
        Ok(value) => assert_eq!(value, &Json::from(2)),
        Err(e) => panic!("Error accessing index: {:?}", e),
    }
    assert!(json.get_index(5).is_err());
}

#[test]
fn test_get_path() {
    let json = Json::Object(HashMap::from_iter(vec![(
        "nested".to_string(),
        Json::Object(HashMap::from_iter(vec![(
            "key".to_string(),
            Json::from("value"),
        )])),
    )]));

    match json.get_path("nested.key") {
        Ok(value) => assert_eq!(value, &Json::from("value")),
        Err(e) => panic!("Error accessing path: {:?}", e),
    }
    assert!(json.get_path("nested.missing").is_err());
}

#[test]
fn test_set_field() {
    let mut json = Json::Object(HashMap::new());
    if let Err(e) = json.set_field("key", "value") {
        panic!("Error setting field: {:?}", e);
    }

    match json.get_field("key") {
        Ok(value) => assert_eq!(value, &Json::from("value")),
        Err(e) => panic!("Error accessing field: {:?}", e),
    }
}

#[test]
fn test_remove_field() {
    let mut json = Json::Object(HashMap::from_iter(vec![(
        "key".to_string(),
        Json::from("value"),
    )]));

    match json.remove_field("key") {
        Ok(value) => assert_eq!(value, Some(Json::from("value"))),
        Err(e) => panic!("Error removing field: {:?}", e),
    }
    assert!(json.get_field("key").is_err());
}

#[test]
fn test_push() {
    let mut json = Json::Array(vec![]);
    if let Err(err) = json.push(1) {
        panic!("Error pushing value: {:?}", err);
    }

    match json.get_index(0) {
        Ok(value) => assert_eq!(value, &Json::from(1)),
        Err(err) => panic!("Error accessing index: {:?}", err),
    }
}

#[test]
fn test_insert_at() {
    let mut json = Json::Array(vec![Json::from(1), Json::from(3)]);
    if let Err(err) = json.insert_at(1, 2) {
        panic!("Error inserting value: {:?}", err);
    }

    match json.get_index(1) {
        Ok(value) => assert_eq!(value, &Json::from(2)),
        Err(err) => panic!("Error accessing index: {:?}", err),
    }
}

#[test]
fn test_remove_at() {
    let mut json = Json::Array(vec![Json::from(1), Json::from(2), Json::from(3)]);
    match json.remove_at(1) {
        Ok(value) => assert_eq!(value, Json::from(2)),
        Err(err) => panic!("Error removing value: {:?}", err),
    }

    match json.get_index(1) {
        Ok(value) => assert_eq!(value, &Json::from(3)),
        Err(err) => panic!("Error accessing index: {:?}", err),
    }
}

#[test]
fn test_is_truthy() {
    assert!(Json::from(true).is_truthy());
    assert!(!Json::Null.is_truthy());
}
