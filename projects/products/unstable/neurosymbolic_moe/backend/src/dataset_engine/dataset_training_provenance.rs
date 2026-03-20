use serde::{Deserialize, Serialize};

use crate::orchestrator::Version;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetTrainingProvenance {
    pub generator: String,
    pub governance_state_version: Version,
    pub governance_state_checksum: String,
    pub runtime_bundle_checksum: String,
    pub dataset_entry_count: usize,
}
