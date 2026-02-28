use std::collections::BTreeMap;
use anyhow::Result;

pub fn check_file(path: &str) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    check_string(&content, path)
}

pub fn check_string(content: &str, label: &str) -> Result<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(content)?;
    let canonical = to_canonical_string(&value)?;
    let original_normalized: serde_json::Value = serde_json::from_str(content)?;
    let original_str = serde_json::to_string(&original_normalized)?;

    if original_str == canonical {
        Ok(vec![])
    } else {
        let diff = crate::canon::canonical_diff::diff_strings(&original_str, &canonical);
        Ok(vec![format!("{}: canonical mismatch\n{}", label, diff)])
    }
}

fn to_canonical_string(value: &serde_json::Value) -> Result<String> {
    let ordered = sort_value(value);
    Ok(serde_json::to_string(&ordered)?)
}

fn sort_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let sorted: BTreeMap<String, serde_json::Value> = map.iter()
                .map(|(k, v)| (k.clone(), sort_value(v)))
                .collect();
            serde_json::Value::Object(sorted.into_iter().collect())
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(sort_value).collect())
        }
        other => other.clone(),
    }
}
