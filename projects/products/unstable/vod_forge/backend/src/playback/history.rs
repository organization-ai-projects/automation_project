use crate::playback::profile_id::ProfileId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub profile_id: ProfileId,
    pub episode_id: String,
    pub completed: bool,
    pub ticks_watched: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn record(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }
}
