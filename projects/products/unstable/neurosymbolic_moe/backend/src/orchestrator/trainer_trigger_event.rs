use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerTriggerEvent {
    pub event_id: u64,
    pub model_version: u64,
    pub training_bundle_checksum: String,
    pub included_entries: usize,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub generated_at: u64,
}
