use serde::{Deserialize, Serialize};

use crate::orchestrator::ModelRegistryEntry;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelRegistry {
    pub entries: Vec<ModelRegistryEntry>,
    pub active_version: Option<u64>,
    pub next_version: u64,
}

impl ModelRegistry {
    pub fn register_candidate(
        &mut self,
        training_bundle_checksum: String,
        included_entries: usize,
        train_samples: usize,
        validation_samples: usize,
        generated_at: u64,
    ) -> u64 {
        let version = self.next_version.max(1);
        self.next_version = version.saturating_add(1);
        self.entries.push(ModelRegistryEntry {
            version,
            training_bundle_checksum,
            included_entries,
            train_samples,
            validation_samples,
            generated_at,
            promoted: false,
        });
        version
    }

    pub fn promote(&mut self, version: u64) -> bool {
        let mut found = false;
        for entry in &mut self.entries {
            if entry.version == version {
                entry.promoted = true;
                found = true;
            }
        }
        if found {
            self.active_version = Some(version);
        }
        found
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn latest_version(&self) -> Option<u64> {
        self.entries.last().map(|entry| entry.version)
    }
}
