// projects/libraries/common_json/src/tests/json_access.rs
use super::test_helpers::TestResult;
use crate::Json;
use crate::JsonObject;
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;

#[test]
fn test_json_access() -> TestResult {
    let mut json = Json::Object(JsonObject::new());
    json.set_field("key", Json::String("value".to_string()))?;

    let value = json.get_field("key")?;
    assert_eq!(value, &Json::String("value".to_string()));
    assert!(json.get_field("nonexistent").is_err());
    Ok(())
}
