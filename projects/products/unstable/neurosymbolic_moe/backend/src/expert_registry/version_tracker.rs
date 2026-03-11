use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, ExpertStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub expert_id: ExpertId,
    pub version: String,
    pub registered_at: u64,
    pub status: ExpertStatus,
}

#[derive(Debug)]
pub struct VersionTracker {
    history: HashMap<ExpertId, Vec<VersionEntry>>,
}

impl VersionTracker {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    pub fn record_version(&mut self, entry: VersionEntry) {
        self.history
            .entry(entry.expert_id.clone())
            .or_default()
            .push(entry);
    }

    pub fn get_history(&self, id: &ExpertId) -> Option<&[VersionEntry]> {
        self.history.get(id).map(|v| v.as_slice())
    }

    pub fn latest_version(&self, id: &ExpertId) -> Option<&VersionEntry> {
        self.history.get(id).and_then(|v| v.last())
    }
}

impl Default for VersionTracker {
    fn default() -> Self {
        Self::new()
    }
}
