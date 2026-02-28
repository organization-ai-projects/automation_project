use crate::replay::search_event::{SearchEvent, SearchEventKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLog {
    pub events: Vec<SearchEvent>,
    pub next_sequence: u64,
}

impl EventLog {
    pub fn push(&mut self, kind: SearchEventKind) {
        let seq = self.next_sequence;
        self.next_sequence += 1;
        self.events.push(SearchEvent {
            sequence: seq,
            kind,
        });
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let json =
            serde_json::to_string_pretty(self).map_err(|e| std::io::Error::other(e.to_string()))?;
        std::fs::write(path, json)
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(|e| std::io::Error::other(e.to_string()))
    }
}
