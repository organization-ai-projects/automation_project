// projects/libraries/common_json/src/tests/json_access_mut.rs
use crate::Json;
use crate::JsonObject;
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;

#[test]
fn test_json_access_mut() -> crate::JsonResult<()> {
    let mut json = Json::Object(JsonObject::new());
    json.set_field("key", Json::String("value".to_string()))?;

    let value = json.get_field("key")?;
    assert_eq!(value, &Json::String("value".to_string()));

    let mut json = Json::Object(JsonObject::new());
    json.set_field("key", Json::String("value".to_string()))?;
    json.remove_field("key")?;
    assert!(json.get_field("key").is_err());
    Ok(())
}
