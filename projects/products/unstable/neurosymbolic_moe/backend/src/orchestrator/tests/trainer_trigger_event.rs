use crate::orchestrator::{TrainerTriggerEvent, Version};

#[test]
fn trainer_trigger_event_fields_roundtrip() {
    let event_id = crate::tests::helpers::protocol_id(1);
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
