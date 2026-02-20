// projects/libraries/common_json/src/merge.rs
use crate::Json;
use crate::json_comparison::JsonComparison;
use crate::merge_strategy::MergeStrategy;
use crate::value::JsonMap;

/// Merges two JSON values using the specified strategy.
///
/// # Strategies
///
/// - `Replace`: Simply returns `source`
/// - `DeepMerge`: Recursively merges objects
/// - `Concat`: Like `DeepMerge` but concatenates arrays
pub fn merge(target: &Json, source: &Json, strategy: MergeStrategy) -> Json {
    let comparison = JsonComparison::compare(target, source);
    match strategy {
        MergeStrategy::Replace => source.clone(),
        MergeStrategy::DeepMerge => deep_merge(target, comparison),
        MergeStrategy::Concat => concat_merge(target, comparison),
    }
}

/// Applies a deep merge based on the `JsonComparison`.
fn deep_merge(target: &Json, comparison: JsonComparison) -> Json {
    match target {
        Json::Object(target_map) => {
            let mut result = target_map.clone();
            for (key, diff) in comparison.object_differences {
                let merged_value = match diff.value_difference {
                    Some((_, updated)) => updated,
                    None => deep_merge(target_map.get(&key).unwrap_or(&Json::Null), diff),
                };
                result.insert(key, merged_value);
            }
            Json::Object(result)
        }
        _ => target.clone(),
    }
}

/// Applies a concatenation merge based on the `JsonComparison`.
fn concat_merge(target: &Json, comparison: JsonComparison) -> Json {
    match target {
        Json::Object(target_map) => {
            let mut result = target_map.clone();
            for (key, diff) in comparison.object_differences {
                let merged_value = match diff.value_difference {
                    Some((_, updated)) => updated,
                    None => concat_merge(target_map.get(&key).unwrap_or(&Json::Null), diff),
                };
                result.insert(key, merged_value);
            }
            Json::Object(result)
        }
        Json::Array(target_arr) => {
            let mut result = target_arr.clone();
            for diff in comparison.array_differences {
                if let Some((_, updated)) = diff.value_difference {
                    result.push(updated);
                }
            }
            Json::Array(result)
        }
        _ => target.clone(),
    }
}

/// Checks if one JSON contains another (for filtering/matching).
///
/// The check is recursive:
/// - For objects: all fields in `needle` must exist in `haystack` with the same values
/// - For arrays: all elements in `needle` must be present in `haystack`
/// - For primitives: strict equality
pub fn contains(haystack: &Json, needle: &Json) -> bool {
    match (haystack, needle) {
        (Json::Object(h), Json::Object(n)) => n
            .iter()
            .all(|(key, value)| h.get(key).is_some_and(|h_value| contains(h_value, value))),
        (Json::Array(h), Json::Array(n)) => n
            .iter()
            .all(|needle_item| h.iter().any(|h_item| contains(h_item, needle_item))),
        _ => haystack == needle,
    }
}

/// Flattens a nested JSON object into keys with dotted notation.
///
/// Transforms a hierarchical structure into a flat object where keys
/// represent the full path to each value.
///
/// # Limitations
///
/// - Only objects are flattened, not arrays
/// - Keys containing dots may cause ambiguities
pub fn flatten(value: &Json) -> Json {
    let mut result = JsonMap::new();
    flatten_recursive(value, String::new(), &mut result);
    Json::Object(result)
}

fn flatten_recursive(value: &Json, prefix: String, result: &mut JsonMap) {
    match value {
        Json::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_recursive(val, new_prefix, result);
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.insert(prefix, value.clone());
            }
        }
    }
}

/// Reconstructs a nested object from keys with dotted notation.
///
/// Inverse operation of [`flatten`].
pub fn unflatten(value: &Json) -> Json {
    match value {
        Json::Object(map) => {
            let mut result = Json::Object(JsonMap::new());
            for (key, val) in map {
                set_nested_value(&mut result, key, val.clone());
            }
            result
        }
        _ => value.clone(),
    }
}

fn set_nested_value(root: &mut Json, path: &str, value: Json) {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = root;

    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Json::Object(map) = current {
                map.insert((*part).to_string(), value);
            }
            return;
        }

        if let Json::Object(map) = current {
            if !map.contains_key(*part) {
                map.insert((*part).to_string(), Json::Object(JsonMap::new()));
            }
            current = map.get_mut(*part).expect("nested key present");
        }
    }
}
