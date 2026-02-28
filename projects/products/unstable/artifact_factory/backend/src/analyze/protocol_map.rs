use crate::input::artifact_input::ArtifactInput;
use std::collections::BTreeMap;

/// A deterministic map of protocol identifiers → source artifact paths.
#[derive(Debug, Clone, Default)]
pub struct ProtocolMap {
    pub entries: BTreeMap<String, Vec<String>>,
}

impl ProtocolMap {
    pub fn build(inputs: &[ArtifactInput]) -> Self {
        let mut entries: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for input in inputs {
            for line in input.content.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("protocol:").or_else(|| trimmed.strip_prefix("Protocol:")) {
                    let proto_id = rest.trim().to_string();
                    if !proto_id.is_empty() {
                        entries.entry(proto_id).or_default().push(input.path.clone());
                    }
                }
            }
        }
        Self { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
