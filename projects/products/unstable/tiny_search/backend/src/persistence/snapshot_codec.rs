use crate::diagnostics::error::Error;
use crate::index::inverted_index::InvertedIndex;
use crate::persistence::index_snapshot::IndexSnapshot;

/// Codec for reading and writing index snapshots to/from files.
pub(crate) struct SnapshotCodec;

impl SnapshotCodec {
    pub(crate) fn save(index: &InvertedIndex, path: &std::path::Path) -> Result<(), Error> {
        let snapshot = IndexSnapshot::from_index(index)?;
        let json =
            common_json::to_string(&snapshot).map_err(|e| Error::Serialization(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| Error::Io(e.to_string()))
    }

    pub(crate) fn load(path: &std::path::Path) -> Result<InvertedIndex, Error> {
        let data = std::fs::read_to_string(path).map_err(|e| Error::Io(e.to_string()))?;
        let snapshot: IndexSnapshot =
            common_json::from_str(&data).map_err(|e| Error::Deserialization(e.to_string()))?;
        snapshot.to_index()
    }

    /// Produce canonical (deterministic) JSON for the index.
    /// BTreeMap ensures sorted keys; we re-serialize via common_json.
    pub(crate) fn canonical_index_json(index: &InvertedIndex) -> Result<String, Error> {
        let json_value =
            common_json::to_json(index).map_err(|e| Error::Serialization(e.to_string()))?;
        let canonical = Self::canonical_json(&json_value);
        Ok(canonical)
    }

    fn canonical_json(json: &common_json::Json) -> String {
        match json {
            common_json::Json::Null => "null".to_string(),
            common_json::Json::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            common_json::Json::Number(n) => {
                let value = n.as_f64();
                if value.fract() == 0.0 && value.abs() < (i64::MAX as f64) {
                    (value as i64).to_string()
                } else {
                    value.to_string()
                }
            }
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
                    .map(|(k, v)| {
                        format!(
                            "{}:{}",
                            Self::escape_json_string(k),
                            Self::canonical_json(v)
                        )
                    })
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
