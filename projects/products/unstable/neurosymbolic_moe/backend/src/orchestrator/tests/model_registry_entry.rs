use crate::orchestrator::ModelRegistryEntry;

#[test]
fn model_registry_entry_fields_roundtrip() {
    let entry = ModelRegistryEntry {
        version: 3,
        training_bundle_checksum: "abc123".to_string(),
        included_entries: 100,
        train_samples: 80,
        validation_samples: 20,
        generated_at: 42,
        promoted: true,
    };
    assert_eq!(entry.version, 3);
    assert_eq!(entry.training_bundle_checksum, "abc123");
    assert_eq!(entry.included_entries, 100);
    assert_eq!(entry.train_samples, 80);
    assert_eq!(entry.validation_samples, 20);
    assert_eq!(entry.generated_at, 42);
    assert!(entry.promoted);
}
