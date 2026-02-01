// projects/libraries/common_json/src/tests/json_comparison.rs
use crate::json::Json;
use crate::json_comparison::JsonComparison;

#[cfg(test)]
#[test]
fn test_json_comparison_new() {
    let comparison = JsonComparison::new();
    assert!(comparison.object_differences.is_empty());
    assert!(comparison.array_differences.is_empty());
    assert!(comparison.value_difference.is_none());
}

#[test]
fn test_json_comparison_compare() {
    let json1 = Json::Null;
    let json2 = Json::Null;
    let comparison = JsonComparison::compare(&json1, &json2);
    assert!(comparison.object_differences.is_empty());
    assert!(comparison.array_differences.is_empty());
    assert!(comparison.value_difference.is_some());
}
