// projects/libraries/layers/domain/versioning/src/revision_log.rs

use crate::release_id::ReleaseId;
use crate::revision_entry::RevisionEntry;
use serde::{Deserialize, Serialize};

/// Manages the complete revision history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionLog {
    entries: Vec<RevisionEntry>,
    project_title: String,
}

impl RevisionLog {
    pub fn initialize(project_title: String) -> Self {
        Self {
            entries: Vec::new(),
            project_title,
        }
    }

    pub fn append_entry(&mut self, entry: RevisionEntry) {
        self.entries.push(entry);
        // Keep sorted by release ID (descending)
        self.entries
            .sort_by(|a, b| b.get_release().cmp(a.get_release()));
    }

    pub fn get_entries(&self) -> &[RevisionEntry] {
        &self.entries
    }

    pub fn get_project_title(&self) -> &str {
        &self.project_title
    }

    pub fn find_entry(&self, release: &ReleaseId) -> Option<&RevisionEntry> {
        self.entries.iter().find(|e| e.get_release() == release)
    }

    pub fn most_recent(&self) -> Option<&RevisionEntry> {
        self.entries.first()
    }
}
