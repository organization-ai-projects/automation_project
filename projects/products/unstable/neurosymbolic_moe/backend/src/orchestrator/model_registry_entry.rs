use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryEntry {
    pub version: u64,
    pub training_bundle_checksum: String,
    pub included_entries: usize,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub generated_at: u64,
    pub promoted: bool,
}
