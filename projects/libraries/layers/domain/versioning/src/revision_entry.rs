// projects/libraries/layers/domain/versioning/src/revision_entry.rs

use crate::modification_entry::ModificationEntry;
use crate::release_id::ReleaseId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A collection of modifications for a specific release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionEntry {
    release: ReleaseId,
    timestamp: DateTime<Utc>,
    modifications: Vec<ModificationEntry>,
    contributors: Vec<String>,
}

impl RevisionEntry {
    pub fn create(release: ReleaseId, timestamp: DateTime<Utc>) -> Self {
        Self {
            release,
            timestamp,
            modifications: Vec::new(),
            contributors: Vec::new(),
        }
    }

    pub fn append_modification(&mut self, entry: ModificationEntry) {
        self.modifications.push(entry);
    }

    pub fn append_contributor(&mut self, name: String) {
        if !self.contributors.contains(&name) {
            self.contributors.push(name);
        }
    }

    pub fn get_release(&self) -> &ReleaseId {
        &self.release
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn get_modifications(&self) -> &[ModificationEntry] {
        &self.modifications
    }

    pub fn get_contributors(&self) -> &[String] {
        &self.contributors
    }
}
