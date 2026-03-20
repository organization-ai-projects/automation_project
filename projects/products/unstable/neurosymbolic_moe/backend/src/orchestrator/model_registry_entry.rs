use crate::orchestrator::version::Version;
use common_time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryEntry {
    pub model_version: Version,
    pub training_bundle_checksum: String,
    pub included_entries: usize,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub generated_at: Timestamp,
    pub promoted: bool,
}
