// projects/libraries/common_json/src/tests/json_access_mut.rs
use crate::Json;
use crate::JsonObject;
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;

#[cfg(test)]
#[test]
fn test_json_access_mut() {
    let mut json = Json::Object(JsonObject::new());
    if let Err(e) = json.set_field("key", Json::String("value".to_string())) {
        panic!("Error setting field: {:?}", e);
    }
    match json.get_field("key") {
        Ok(value) => assert_eq!(value, &Json::String("value".to_string())),
        Err(e) => panic!("Error accessing field: {:?}", e),
    }

    let mut json = Json::Object(JsonObject::new());
    if let Err(e) = json.set_field("key", Json::String("value".to_string())) {
        panic!("Error setting field: {:?}", e);
    }
    if let Err(e) = json.remove_field("key") {
        panic!("Error removing field: {:?}", e);
    }
    assert!(json.get_field("key").is_err());
}
