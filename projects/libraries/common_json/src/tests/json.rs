// projects/libraries/common_json/src/tests/json.rs
use crate::json::to_json_string_pretty;

#[test]
fn test_to_json_string_pretty() -> crate::JsonResult<()> {
    let value = vec!["test", "json"];
    let json = to_json_string_pretty(&value)?;
    assert!(json.contains("test"));
    assert!(json.contains("json"));
    Ok(())
}
