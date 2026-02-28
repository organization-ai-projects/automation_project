use crate::input::artifact_input::ArtifactInput;
use std::collections::BTreeMap;

/// A deterministic map of event names → source artifact paths.
#[derive(Debug, Clone, Default)]
pub struct EventMap {
    pub entries: BTreeMap<String, Vec<String>>,
}

impl EventMap {
    pub fn build(inputs: &[ArtifactInput]) -> Self {
        let mut entries: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for input in inputs {
            for line in input.content.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("event:").or_else(|| trimmed.strip_prefix("Event:")) {
                    let event_name = rest.trim().to_string();
                    if !event_name.is_empty() {
                        entries.entry(event_name).or_default().push(input.path.clone());
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
