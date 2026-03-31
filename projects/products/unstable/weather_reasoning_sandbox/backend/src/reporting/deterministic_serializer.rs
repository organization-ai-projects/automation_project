use std::collections::BTreeMap;
use std::fmt::Write;

use serde::Serialize;

/// A JSON value representation that uses BTreeMap for deterministic key ordering.
enum SortedJson {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<SortedJson>),
    Object(BTreeMap<String, SortedJson>),
}

pub struct DeterministicSerializer;

impl DeterministicSerializer {
    pub fn serialize_canonical<T: Serialize>(value: &T) -> Result<String, String> {
        let json = common_json::to_value(value).map_err(|e| format!("serialization error: {e}"))?;
        let sorted = Self::to_sorted(&json);
        Ok(Self::to_pretty_string(&sorted))
    }

    fn to_sorted(value: &common_json::Json) -> SortedJson {
        match value {
            common_json::Json::Null => SortedJson::Null,
            common_json::Json::Bool(b) => SortedJson::Bool(*b),
            common_json::Json::Number(n) => {
                let f = n.as_f64();
                let max_safe_int = (1i64 << 53) as f64;
                if f.fract() == 0.0 && f.abs() < max_safe_int {
                    SortedJson::Integer(f as i64)
                } else {
                    SortedJson::Float(f)
                }
            }
            common_json::Json::String(s) => SortedJson::String(s.clone()),
            common_json::Json::Array(arr) => {
                SortedJson::Array(arr.iter().map(|v| Self::to_sorted(v)).collect())
            }
            common_json::Json::Object(map) => {
                let sorted: BTreeMap<String, SortedJson> = map
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::to_sorted(v)))
                    .collect();
                SortedJson::Object(sorted)
            }
        }
    }

    fn to_pretty_string(value: &SortedJson) -> String {
        let mut output = String::new();
        Self::append(value, &mut output, 0);
        output
    }

    fn append(value: &SortedJson, out: &mut String, indent: usize) {
        match value {
            SortedJson::Null => out.push_str("null"),
            SortedJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            SortedJson::Integer(n) => {
                let _ = write!(out, "{n}");
            }
            SortedJson::Float(f) => {
                let _ = write!(out, "{f}");
            }
            SortedJson::String(s) => Self::append_escaped_string(out, s),
            SortedJson::Array(arr) => {
                if arr.is_empty() {
                    out.push_str("[]");
                } else {
                    out.push_str("[\n");
                    for (i, v) in arr.iter().enumerate() {
                        Self::push_indent(out, indent + 2);
                        Self::append(v, out, indent + 2);
                        if i + 1 < arr.len() {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    Self::push_indent(out, indent);
                    out.push(']');
                }
            }
            SortedJson::Object(map) => {
                if map.is_empty() {
                    out.push_str("{}");
                } else {
                    out.push_str("{\n");
                    let len = map.len();
                    for (i, (k, v)) in map.iter().enumerate() {
                        Self::push_indent(out, indent + 2);
                        Self::append_escaped_string(out, k);
                        out.push_str(": ");
                        Self::append(v, out, indent + 2);
                        if i + 1 < len {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    Self::push_indent(out, indent);
                    out.push('}');
                }
            }
        }
    }

    fn push_indent(out: &mut String, indent: usize) {
        for _ in 0..indent {
            out.push(' ');
        }
    }

    fn append_escaped_string(out: &mut String, s: &str) {
        out.push('"');
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{08}' => out.push_str("\\b"),
                '\u{0C}' => out.push_str("\\f"),
                c if c <= '\u{1F}' => {
                    let _ = write!(out, "\\u{:04x}", c as u32);
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }
}
