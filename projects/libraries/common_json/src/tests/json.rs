// projects/libraries/common_json/src/tests/json.rs
use crate::json::to_json_string_pretty;

#[cfg(test)]
#[test]
fn test_to_json_string_pretty() {
    let value = vec!["test", "json"];
    let result = to_json_string_pretty(&value);
    match result {
        Ok(json) => {
            assert!(json.contains("test"));
            assert!(json.contains("json"));
        }
        Err(e) => panic!("Error converting to JSON: {:?}", e),
    }
}
