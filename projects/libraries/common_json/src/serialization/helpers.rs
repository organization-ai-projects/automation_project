// projects/libraries/common_json/src/serialization/helpers.rs
use crate::Json;
use crate::json_error::{JsonError, JsonErrorCode, JsonResult};
use crate::serialization::const_values::INDENT_WIDTH;
use crate::serialization::json_serializer::JsonSerializer;
use serde::ser::Serialize;
use std::fmt::Write as FmtWrite;

pub(crate) fn serialize_to_json<T: Serialize>(value: &T) -> Result<Json, JsonError> {
    value.serialize(JsonSerializer)
}

pub(crate) fn json_to_string(json: &Json, pretty: bool) -> JsonResult<String> {
    let mut output = String::new();
    append_json(&mut output, json, pretty, 0)?;
    Ok(output)
}

fn append_json(output: &mut String, json: &Json, pretty: bool, indent: usize) -> JsonResult<()> {
    match json {
        Json::Null => output.push_str("null"),
        Json::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
        Json::Number(number) => {
            let value = number.as_f64();
            if !value.is_finite() {
                return Err(JsonError::new(JsonErrorCode::Serialize));
            }
            output.push_str(&value.to_string());
        }
        Json::String(value) => push_escaped_string(output, value),
        Json::Array(values) => {
            if values.is_empty() {
                output.push_str("[]");
            } else {
                output.push('[');
                if pretty {
                    output.push('\n');
                }
                for (idx, value) in values.iter().enumerate() {
                    if pretty {
                        push_indent(output, indent + INDENT_WIDTH);
                    }
                    append_json(output, value, pretty, indent + INDENT_WIDTH)?;
                    if idx + 1 < values.len() {
                        output.push(',');
                    }
                    if pretty {
                        output.push('\n');
                    }
                }
                if pretty {
                    push_indent(output, indent);
                }
                output.push(']');
            }
        }
        Json::Object(map) => {
            if map.is_empty() {
                output.push_str("{}");
            } else {
                output.push('{');
                if pretty {
                    output.push('\n');
                }
                for (idx, (key, value)) in map.iter().enumerate() {
                    if pretty {
                        push_indent(output, indent + INDENT_WIDTH);
                    }
                    push_escaped_string(output, key);
                    output.push(':');
                    if pretty {
                        output.push(' ');
                    }
                    append_json(output, value, pretty, indent + INDENT_WIDTH)?;
                    if idx + 1 < map.len() {
                        output.push(',');
                    }
                    if pretty {
                        output.push('\n');
                    }
                }
                if pretty {
                    push_indent(output, indent);
                }
                output.push('}');
            }
        }
    }
    Ok(())
}

fn push_indent(output: &mut String, indent: usize) {
    output.push_str(&" ".repeat(indent));
}

fn push_escaped_string(output: &mut String, value: &str) {
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            ch if ch <= '\u{1F}' => {
                let _ = write!(output, "\\u{:04x}", ch as u32);
            }
            _ => output.push(ch),
        }
    }
    output.push('"');
}
