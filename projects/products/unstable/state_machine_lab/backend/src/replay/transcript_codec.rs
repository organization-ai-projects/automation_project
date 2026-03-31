use crate::diagnostics::error::BackendError;
use crate::replay::transcript::Transcript;
use common_json::Json;

pub struct TranscriptCodec;

impl TranscriptCodec {
    pub fn encode(transcript: &Transcript) -> Result<String, BackendError> {
        to_canonical_string(transcript).map_err(BackendError::Codec)
    }

    pub fn decode(raw: &str) -> Result<Transcript, BackendError> {
        common_json::from_json_str(raw).map_err(|e| BackendError::Codec(e.to_string()))
    }
}

fn to_canonical_string<T: serde::Serialize>(value: &T) -> Result<String, String> {
    let json = common_json::to_json(value).map_err(|e| e.to_string())?;
    Ok(canonical_json_string(&json))
}

fn canonical_json_string(json: &Json) -> String {
    match json {
        Json::Null => "null".to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Number(n) => common_json::to_json_string(n).unwrap_or_else(|_| "0".to_string()),
        Json::String(s) => encode_json_string(s),
        Json::Array(items) => {
            let parts: Vec<String> = items.iter().map(canonical_json_string).collect();
            format!("[{}]", parts.join(","))
        }
        Json::Object(map) => {
            let mut entries: Vec<(&String, &Json)> = map.iter().collect();
            entries.sort_by_key(|(k, _)| k.as_str());
            let parts: Vec<String> = entries
                .iter()
                .map(|(k, v)| format!("{}:{}", encode_json_string(k), canonical_json_string(v)))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

fn encode_json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::step_result::StepResult;
    use crate::model::event_id::EventId;
    use crate::model::machine_id::MachineId;
    use crate::model::state_id::StateId;
    use std::collections::BTreeMap;

    fn sample_transcript() -> Transcript {
        Transcript {
            machine_id: MachineId("test".into()),
            seed: Some(42),
            steps: vec![StepResult {
                step: 1,
                previous_state: StateId("off".into()),
                event: EventId("flip".into()),
                next_state: StateId("on".into()),
                variables: BTreeMap::new(),
            }],
            final_state: "on".to_string(),
        }
    }

    #[test]
    fn transcript_roundtrip() {
        let t = sample_transcript();
        let encoded = TranscriptCodec::encode(&t).unwrap();
        let decoded = TranscriptCodec::decode(&encoded).unwrap();
        assert_eq!(t, decoded);
    }

    #[test]
    fn transcript_deterministic_encoding() {
        let t = sample_transcript();
        let e1 = TranscriptCodec::encode(&t).unwrap();
        let e2 = TranscriptCodec::encode(&t).unwrap();
        assert_eq!(e1, e2);
    }
}
