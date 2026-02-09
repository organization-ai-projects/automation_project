// projects/libraries/common_json/src/tests/access.rs
use crate::Json;
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;
use std::collections::HashMap;

#[test]
fn test_get_field() -> crate::JsonResult<()> {
    let json = Json::Object(HashMap::from_iter(vec![(
        "key".to_string(),
        Json::from("value"),
    )]));

    let value = json.get_field("key")?;
    assert_eq!(value, &Json::from("value"));
    assert!(json.get_field("missing").is_err());
    Ok(())
}

#[test]
fn test_get_index() -> crate::JsonResult<()> {
    let json = Json::Array(vec![Json::from(1), Json::from(2), Json::from(3)]);

    let value = json.get_index(1)?;
    assert_eq!(value, &Json::from(2));
    assert!(json.get_index(5).is_err());
    Ok(())
}

#[test]
fn test_get_path() -> crate::JsonResult<()> {
    let json = Json::Object(HashMap::from_iter(vec![(
        "nested".to_string(),
        Json::Object(HashMap::from_iter(vec![(
            "key".to_string(),
            Json::from("value"),
        )])),
    )]));

    let value = json.get_path("nested.key")?;
    assert_eq!(value, &Json::from("value"));
    assert!(json.get_path("nested.missing").is_err());
    Ok(())
}

#[test]
fn test_set_field() -> crate::JsonResult<()> {
    let mut json = Json::Object(HashMap::new());
    json.set_field("key", "value")?;

    let value = json.get_field("key")?;
    assert_eq!(value, &Json::from("value"));
    Ok(())
}

#[test]
fn test_remove_field() -> crate::JsonResult<()> {
    let mut json = Json::Object(HashMap::from_iter(vec![(
        "key".to_string(),
        Json::from("value"),
    )]));

    let value = json.remove_field("key")?;
    assert_eq!(value, Some(Json::from("value")));
    assert!(json.get_field("key").is_err());
    Ok(())
}

#[test]
fn test_push() -> crate::JsonResult<()> {
    let mut json = Json::Array(vec![]);
    json.push(1)?;

    let value = json.get_index(0)?;
    assert_eq!(value, &Json::from(1));
    Ok(())
}

#[test]
fn test_insert_at() -> crate::JsonResult<()> {
    let mut json = Json::Array(vec![Json::from(1), Json::from(3)]);
    json.insert_at(1, 2)?;

    let value = json.get_index(1)?;
    assert_eq!(value, &Json::from(2));
    Ok(())
}

#[test]
fn test_remove_at() -> crate::JsonResult<()> {
    let mut json = Json::Array(vec![Json::from(1), Json::from(2), Json::from(3)]);
    let removed = json.remove_at(1)?;
    assert_eq!(removed, Json::from(2));

    let value = json.get_index(1)?;
    assert_eq!(value, &Json::from(3));
    Ok(())
}

#[test]
fn test_is_truthy() {
    assert!(Json::from(true).is_truthy());
    assert!(!Json::Null.is_truthy());
}
