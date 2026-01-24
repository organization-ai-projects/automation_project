// projects/libraries/common_json/src/json_diff.rs
use std::collections::HashMap;

use crate::json::Json;

/// Represents the result of a JSON comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonComparison {
    /// Differences in object fields.
    pub object_differences: HashMap<String, JsonComparison>,
    /// Differences in array elements.
    pub array_differences: Vec<JsonComparison>,
    /// Differences in primitive values.
    pub value_difference: Option<(Json, Json)>,
}

impl JsonComparison {
    /// Creates a new `JsonComparison` with no differences.
    pub fn new() -> Self {
        Self {
            object_differences: HashMap::new(),
            array_differences: Vec::new(),
            value_difference: None,
        }
    }

    /// Compares two JSON values and returns a `JsonComparison`.
    pub fn compare(json1: &Json, json2: &Json) -> Self {
        match (json1, json2) {
            (Json::Object(obj1), Json::Object(obj2)) => {
                let mut object_differences = HashMap::new();
                for key in obj1.keys().chain(obj2.keys()) {
                    let value1 = obj1.get(key);
                    let value2 = obj2.get(key);
                    if value1 != value2 {
                        object_differences.insert(
                            key.clone(),
                            JsonComparison::compare(
                                value1.unwrap_or(&Json::Null),
                                value2.unwrap_or(&Json::Null),
                            ),
                        );
                    }
                }
                Self {
                    object_differences,
                    ..Self::new()
                }
            }
            (Json::Array(arr1), Json::Array(arr2)) => {
                let mut array_differences = Vec::new();
                let max_len = arr1.len().max(arr2.len());
                for i in 0..max_len {
                    let value1 = arr1.get(i).unwrap_or(&Json::Null);
                    let value2 = arr2.get(i).unwrap_or(&Json::Null);
                    if value1 != value2 {
                        array_differences.push(JsonComparison::compare(value1, value2));
                    }
                }
                Self {
                    array_differences,
                    ..Self::new()
                }
            }
            _ => Self {
                value_difference: Some((json1.clone(), json2.clone())),
                ..Self::new()
            },
        }
    }
}

impl Default for JsonComparison {
    fn default() -> Self {
        Self::new()
    }
}
