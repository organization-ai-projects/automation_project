use crate::orchestrator::{TrainerTriggerEvent, Version};
use protocol::ProtocolId;
use std::str::FromStr;

#[test]
fn trainer_trigger_event_fields_roundtrip() {
    let event_id = ProtocolId::from_str("00000000000000000000000000000001")
        .expect("test protocol id should be valid fixed hex");
    let event = TrainerTriggerEvent {
        event_id,
        model_version: Version::new(3, 0, 0),
        training_bundle_checksum: "bundle-xyz".to_string(),
        included_entries: 120,
        train_samples: 96,
        validation_samples: 24,
        generated_at: 1000,
        delivery_attempts: 2,
        last_attempted_at: Some(1234),
    };
    assert_eq!(event.event_id, event_id);
    assert_eq!(event.model_version, Version::new(3, 0, 0));
    assert_eq!(event.training_bundle_checksum, "bundle-xyz");
    assert_eq!(event.included_entries, 120);
    assert_eq!(event.train_samples, 96);
    assert_eq!(event.validation_samples, 24);
    assert_eq!(event.generated_at, 1000);
    assert_eq!(event.delivery_attempts, 2);
    assert_eq!(event.last_attempted_at, Some(1234));
}
