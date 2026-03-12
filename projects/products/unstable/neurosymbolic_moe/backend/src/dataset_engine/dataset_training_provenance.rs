use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetTrainingProvenance {
    pub generator: String,
    pub governance_state_version: u64,
    pub governance_state_checksum: String,
    pub runtime_bundle_checksum: String,
    pub dataset_entry_count: usize,
}
