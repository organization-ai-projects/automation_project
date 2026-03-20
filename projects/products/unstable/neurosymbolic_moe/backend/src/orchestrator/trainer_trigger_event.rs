use common_time::{Timestamp, current_timestamp_ms};
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

impl TrainerTriggerEvent {
    pub fn new(
        event_id: ProtocolId,
        model_version: Version,
        training_bundle_checksum: String,
        included_entries: usize,
        train_samples: usize,
        validation_samples: usize,
    ) -> Self {
        Self {
            event_id,
            model_version,
            training_bundle_checksum,
            included_entries,
            train_samples,
            validation_samples,
            generated_at: current_timestamp_ms(),
            delivery_attempts: 0,
            last_attempted_at: None,
        }
    }

    pub fn update_last_attempted(&mut self) {
        self.last_attempted_at = Some(current_timestamp_ms());
    }
}
