// projects/libraries/common_json/src/tests/json.rs
use crate::json::to_json_string_pretty;
use super::test_helpers::TestResult;

#[test]
fn test_to_json_string_pretty() -> TestResult {
    let value = vec!["test", "json"];
    let json = to_json_string_pretty(&value)?;
    assert!(json.contains("test"));
    assert!(json.contains("json"));
    Ok(())
}
