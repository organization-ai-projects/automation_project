use std::collections::HashMap;

use super::version_entry::VersionEntry;
use crate::moe_core::ExpertId;

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
