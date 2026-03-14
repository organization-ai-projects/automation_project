use crate::orchestrator::TrainerTriggerEvent;

#[test]
fn trainer_trigger_event_fields_roundtrip() {
    let event = TrainerTriggerEvent {
        event_id: 7,
        model_version: 3,
        training_bundle_checksum: "bundle-xyz".to_string(),
        included_entries: 120,
        train_samples: 96,
        validation_samples: 24,
        generated_at: 1000,
    };
    assert_eq!(event.event_id, 7);
    assert_eq!(event.model_version, 3);
    assert_eq!(event.training_bundle_checksum, "bundle-xyz");
    assert_eq!(event.included_entries, 120);
    assert_eq!(event.train_samples, 96);
    assert_eq!(event.validation_samples, 24);
    assert_eq!(event.generated_at, 1000);
}
