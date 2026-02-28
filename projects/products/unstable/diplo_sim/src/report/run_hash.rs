use sha2::{Sha256, Digest};
use common_json::Json;

/// Compute SHA-256 of bytes and return hex string.
pub fn compute_run_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Compute run hash from a JSON string.
pub fn compute_run_hash_from_json(json: &str) -> String {
    compute_run_hash(json.as_bytes())
}

/// Convert a Json value to a canonical string with alphabetically sorted object keys.
/// This ensures deterministic output regardless of HashMap iteration order.
pub fn canonical_json_string(v: &Json) -> String {
    match v {
        Json::Null => "null".to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Number(n) => {
            // Use serde round-trip to get the number as a string
            let s = common_json::to_json_string(n)
                .unwrap_or_else(|_| "0".to_string());
            s
        }
        Json::String(s) => {
            // Escape JSON string
            let mut out = String::with_capacity(s.len() + 2);
            out.push('"');
            for ch in s.chars() {
                match ch {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c => out.push(c),
                }
            }
            out.push('"');
            out
        }
        Json::Array(arr) => {
            let items: Vec<String> = arr.iter().map(canonical_json_string).collect();
            format!("[{}]", items.join(","))
        }
        Json::Object(map) => {
            // Sort keys alphabetically for determinism
            let mut pairs: Vec<(&String, &Json)> = map.iter().collect();
            pairs.sort_by_key(|(k, _)| k.as_str());
            let items: Vec<String> = pairs
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}:{}",
                        canonical_json_string(&Json::String(k.to_string())),
                        canonical_json_string(v)
                    )
                })
                .collect();
            format!("{{{}}}", items.join(","))
        }
    }
}

/// Compute run hash from a serializable value using canonical JSON (sorted keys).
pub fn compute_canonical_run_hash<T: serde::Serialize>(value: &T) -> String {
    let json_val = common_json::to_json(value)
        .expect("Failed to serialize value for run_hash computation");
    let canonical = canonical_json_string(&json_val);
    compute_run_hash_from_json(&canonical)
}
