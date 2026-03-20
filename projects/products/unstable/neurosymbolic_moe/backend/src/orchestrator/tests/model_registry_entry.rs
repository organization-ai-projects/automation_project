use common_time::Timestamp;

use crate::orchestrator::{ModelRegistryEntry, Version};

#[test]
fn model_registry_entry_fields_roundtrip() {
    let entry = ModelRegistryEntry {
        model_version: Version::new(1, 0, 0),
        training_bundle_checksum: "abc123".to_string(),
        included_entries: 100,
        train_samples: 80,
        validation_samples: 20,
        generated_at: Timestamp::default(),
        promoted: true,
    };
    assert_eq!(entry.model_version, Version::new(1, 0, 0));
    assert_eq!(entry.training_bundle_checksum, "abc123");
    assert_eq!(entry.included_entries, 100);
    assert_eq!(entry.train_samples, 80);
    assert_eq!(entry.validation_samples, 20);
    assert_eq!(entry.generated_at, Timestamp::default());
    assert!(entry.promoted);
}
