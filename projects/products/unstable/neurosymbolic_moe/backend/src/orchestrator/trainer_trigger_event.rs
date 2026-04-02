use common_time::Timestamp;
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use crate::orchestrator::Version;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerTriggerEvent {
    pub event_id: ProtocolId,
    pub model_version: Version,
    pub training_bundle_checksum: String,
    pub included_entries: usize,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub generated_at: Timestamp,
    #[serde(default)]
    pub delivery_attempts: u32,
    #[serde(default)]
    pub last_attempted_at: Option<Timestamp>,
}

impl TrainerTriggerEvent {}
