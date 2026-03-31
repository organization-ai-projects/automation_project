use crate::diagnostics::Error;
use crate::events::StoryEvent;
use crate::report::StoryReport;

pub struct JsonExporter;

impl JsonExporter {
    pub fn export(report: &StoryReport, events: &[StoryEvent]) -> Result<String, Error> {
        let export = ExportPayload {
            report: report.clone(),
            events: events.to_vec(),
        };
        let json_value =
            common_json::to_json(&export).map_err(|e| Error::Serialization(e.to_string()))?;
        Ok(canonical_json_pretty(&json_value, 0))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ExportPayload {
    report: StoryReport,
    events: Vec<StoryEvent>,
}

fn canonical_json_pretty(json: &common_json::Json, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let inner_indent = "  ".repeat(indent + 1);
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
        common_json::Json::String(s) => escape_json_string(s),
        common_json::Json::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let items: Vec<String> = arr
                .iter()
                .map(|v| format!("{}{}", inner_indent, canonical_json_pretty(v, indent + 1)))
                .collect();
            format!("[\n{}\n{}]", items.join(",\n"), indent_str)
        }
        common_json::Json::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let mut entries: Vec<(&String, &common_json::Json)> = map.iter().collect();
            entries.sort_by_key(|(k, _)| (*k).clone());
            let items: Vec<String> = entries
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}{}: {}",
                        inner_indent,
                        escape_json_string(k),
                        canonical_json_pretty(v, indent + 1)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", items.join(",\n"), indent_str)
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
