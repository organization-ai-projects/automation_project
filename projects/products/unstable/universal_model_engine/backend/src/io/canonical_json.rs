use common_json::Json;

pub fn to_canonical_string<T: serde::Serialize>(value: &T) -> Result<String, String> {
    let json = common_json::to_json(value).map_err(|error| error.to_string())?;
    Ok(canonical_json_string(&json))
}

fn canonical_json_string(json: &Json) -> String {
    match json {
        Json::Null => "null".to_string(),
        Json::Bool(boolean) => boolean.to_string(),
        Json::Number(number) => {
            common_json::to_json_string(number).unwrap_or_else(|_| "0".to_string())
        }
        Json::String(text) => encode_json_string(text),
        Json::Array(items) => {
            let parts: Vec<String> = items.iter().map(canonical_json_string).collect();
            format!("[{}]", parts.join(","))
        }
        Json::Object(map) => {
            let mut entries: Vec<(&String, &Json)> = map.iter().collect();
            entries.sort_by_key(|(key, _)| key.as_str());
            let parts: Vec<String> = entries
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}:{}",
                        encode_json_string(key),
                        canonical_json_string(value)
                    )
                })
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

fn encode_json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for char_value in text.chars() {
        match char_value {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}
