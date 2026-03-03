// projects/products/unstable/vod_forge/backend/src/playback/history.rs
use crate::playback::HistoryEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn record(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }
}
