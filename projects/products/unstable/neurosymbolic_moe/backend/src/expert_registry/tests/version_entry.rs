use crate::expert_registry::VersionEntry;
use crate::moe_core::{ExpertId, ExpertStatus};

#[test]
fn version_entry_fields_round_trip() {
    let entry = VersionEntry {
        expert_id: ExpertId::new("expert-v"),
        version: "1.2.3".to_string(),
        registered_at: 77,
        status: ExpertStatus::Active,
    };
    assert_eq!(entry.expert_id.as_str(), "expert-v");
    assert_eq!(entry.version, "1.2.3");
    assert_eq!(entry.registered_at, 77);
    assert!(matches!(entry.status, ExpertStatus::Active));
}
