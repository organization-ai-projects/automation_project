use common_time::Timestamp;
use serde::{Deserialize, Serialize};

use crate::orchestrator::ModelRegistryEntry;
use crate::orchestrator::version::Version;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelRegistry {
    pub entries: Vec<ModelRegistryEntry>,
    pub active_model_version: Option<Version>,
    pub next_model_version: Version,
}

impl ModelRegistry {
    pub fn register_candidate(
        &mut self,
        training_bundle_checksum: String,
        included_entries: usize,
        train_samples: usize,
        validation_samples: usize,
        generated_at: Timestamp,
    ) -> Version {
        let model_version = self.next_model_version.clone();
        self.next_model_version.increment_patch();
        self.entries.push(ModelRegistryEntry {
            model_version: model_version.clone(),
            training_bundle_checksum,
            included_entries,
            train_samples,
            validation_samples,
            generated_at,
            promoted: false,
        });
        model_version
    }

    pub fn promote(&mut self, model_version: Version) -> bool {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.model_version == model_version)
        {
            entry.promoted = true;
            self.active_model_version = Some(model_version);
            true
        } else {
            false
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn latest_model_version(&self) -> Option<Version> {
        self.entries.last().map(|entry| entry.model_version.clone())
    }
}
