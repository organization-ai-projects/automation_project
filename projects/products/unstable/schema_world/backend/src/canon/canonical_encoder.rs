use crate::snapshots::snapshot::Snapshot;
use common_json::Json;
use std::collections::HashMap;

pub struct CanonicalEncoder;

impl CanonicalEncoder {
    pub fn encode_value(value: &Json) -> Result<Vec<u8>, String> {
        let canonical = canonicalize_value(value);
        canonical_to_string(&canonical).map(|s| s.into_bytes())
    }

    pub fn encode_snapshot(snapshot: &Snapshot) -> Result<Vec<u8>, String> {
        let value =
            common_json::to_value(snapshot).map_err(|e| format!("snapshot encode failed: {e}"))?;
        Self::encode_value(&value)
    }
}

pub fn canonicalize_value(value: &Json) -> Json {
    match value {
        Json::Object(map) => {
            let mut ordered = HashMap::new();
            for (key, inner) in map {
                ordered.insert(key.clone(), canonicalize_value(inner));
            }
            Json::Object(ordered)
        }
        Json::Array(items) => Json::Array(items.iter().map(canonicalize_value).collect()),
        _ => value.clone(),
    }
}

fn canonical_to_string(value: &Json) -> Result<String, String> {
    match value {
        Json::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            let mut out = String::from("{");
            for (idx, key) in keys.iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                let key_json = common_json::to_string(key)
                    .map_err(|e| format!("canonical encode failed: {e}"))?;
                out.push_str(&key_json);
                out.push(':');
                let Some(inner) = map.get(*key) else {
                    return Err("canonical encode failed: missing key".to_string());
                };
                out.push_str(&canonical_to_string(inner)?);
            }
            out.push('}');
            Ok(out)
        }
        Json::Array(items) => {
            let mut out = String::from("[");
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                out.push_str(&canonical_to_string(item)?);
            }
            out.push(']');
            Ok(out)
        }
        _ => common_json::to_string(value).map_err(|e| format!("canonical encode failed: {e}")),
    }
}
