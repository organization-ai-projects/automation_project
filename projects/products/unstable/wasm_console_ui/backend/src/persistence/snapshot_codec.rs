use sha2::{Digest, Sha256};

use crate::app::app_state::AppState;
use crate::diagnostics::error::Error;
use crate::persistence::ui_snapshot::UiSnapshot;

/// Codec for canonical snapshot serialization and deserialization.
pub struct SnapshotCodec;

impl SnapshotCodec {
    /// Export the current state to a canonical JSON snapshot string.
    pub fn export(state: &AppState) -> Result<String, Error> {
        let state_json_value =
            common_json::to_json(state).map_err(|e| Error::Serialization(e.to_string()))?;
        let canonical = Self::canonical_json(&state_json_value);
        let checksum = Self::compute_checksum(&canonical);
        let snapshot = UiSnapshot::new(canonical, checksum);
        common_json::to_string(&snapshot).map_err(|e| Error::Serialization(e.to_string()))
    }

    /// Import a snapshot from a JSON string and return the reconstructed state.
    pub fn import(data: &str) -> Result<AppState, Error> {
        let snapshot: UiSnapshot =
            common_json::from_str(data).map_err(|e| Error::Deserialization(e.to_string()))?;
        let expected_checksum = Self::compute_checksum(&snapshot.state_json);
        if snapshot.checksum != expected_checksum {
            return Err(Error::ChecksumMismatch {
                expected: expected_checksum,
                actual: snapshot.checksum,
            });
        }
        common_json::from_str(&snapshot.state_json)
            .map_err(|e| Error::Deserialization(e.to_string()))
    }

    fn compute_checksum(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Produce a deterministic JSON string with sorted object keys.
    fn canonical_json(json: &common_json::Json) -> String {
        match json {
            common_json::Json::Null => "null".to_string(),
            common_json::Json::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            common_json::Json::Number(n) => n.as_f64().to_string(),
            common_json::Json::String(s) => Self::escape_json_string(s),
            common_json::Json::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| Self::canonical_json(v)).collect();
                format!("[{}]", items.join(","))
            }
            common_json::Json::Object(map) => {
                let mut entries: Vec<(&String, &common_json::Json)> = map.iter().collect();
                entries.sort_by_key(|(k, _)| (*k).clone());
                let items: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{}:{}", Self::escape_json_string(k), Self::canonical_json(v)))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }

    fn escape_json_string(s: &str) -> String {
        let mut output = String::with_capacity(s.len() + 2);
        output.push('"');
        for ch in s.chars() {
            match ch {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                '\u{08}' => output.push_str("\\b"),
                '\u{0C}' => output.push_str("\\f"),
                ch if ch <= '\u{1F}' => {
                    use std::fmt::Write;
                    let _ = write!(output, "\\u{:04x}", ch as u32);
                }
                _ => output.push(ch),
            }
        }
        output.push('"');
        output
    }
}
