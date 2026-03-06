use anyhow::Result;
use common_json::Json;
use std::collections::BTreeMap;

pub fn check_file(path: &str) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    check_string(&content, path)
}

pub fn check_string(content: &str, label: &str) -> Result<Vec<String>> {
    let value: Json = common_json::from_json_str(content)?;
    let canonical = to_canonical_string(&value)?;
    let original_normalized: Json = common_json::from_json_str(content)?;
    let original_str = common_json::to_string(&original_normalized)?;

    if original_str == canonical {
        Ok(vec![])
    } else {
        let diff = crate::canon::canonical_diff::diff_strings(&original_str, &canonical);
        Ok(vec![format!("{}: canonical mismatch\n{}", label, diff)])
    }
}

fn to_canonical_string(value: &Json) -> Result<String> {
    let mut output = String::new();
    append_canonical_json(&mut output, value)?;
    Ok(output)
}

fn append_canonical_json(output: &mut String, value: &Json) -> Result<()> {
    match value {
        Json::Object(map) => {
            output.push('{');
            let sorted: BTreeMap<&str, &Json> = map.iter().map(|(k, v)| (k.as_str(), v)).collect();
            for (index, (key, nested)) in sorted.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str(&common_json::to_string(key)?);
                output.push(':');
                append_canonical_json(output, nested)?;
            }
            output.push('}');
        }
        Json::Array(items) => {
            output.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                append_canonical_json(output, item)?;
            }
            output.push(']');
        }
        _ => output.push_str(&common_json::to_string(value)?),
    }
    Ok(())
}
