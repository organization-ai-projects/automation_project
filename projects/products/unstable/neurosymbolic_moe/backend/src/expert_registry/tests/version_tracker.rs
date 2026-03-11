use crate::expert_registry::{VersionEntry, VersionTracker};
use crate::moe_core::{ExpertId, ExpertStatus};

#[test]
fn records_and_reads_history() {
    let mut tracker = VersionTracker::new();
    let expert_id = ExpertId::new("expert-a");

    tracker.record_version(VersionEntry {
        expert_id: expert_id.clone(),
        version: "1.0.0".to_string(),
        registered_at: 100,
        status: ExpertStatus::Active,
    });
    tracker.record_version(VersionEntry {
        expert_id: expert_id.clone(),
        version: "1.1.0".to_string(),
        registered_at: 200,
        status: ExpertStatus::Active,
    });

    let history = tracker
        .get_history(&expert_id)
        .expect("history should exist for recorded expert");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].version, "1.0.0");
    assert_eq!(history[1].version, "1.1.0");
}

#[test]
fn latest_version_returns_last_entry() {
    let mut tracker = VersionTracker::new();
    let expert_id = ExpertId::new("expert-b");

    tracker.record_version(VersionEntry {
        expert_id: expert_id.clone(),
        version: "2.0.0".to_string(),
        registered_at: 300,
        status: ExpertStatus::Inactive,
    });
    tracker.record_version(VersionEntry {
        expert_id: expert_id.clone(),
        version: "2.1.0".to_string(),
        registered_at: 400,
        status: ExpertStatus::Active,
    });

    let latest = tracker
        .latest_version(&expert_id)
        .expect("latest version should exist");
    assert_eq!(latest.version, "2.1.0");
    assert_eq!(latest.registered_at, 400);
    assert!(matches!(latest.status, ExpertStatus::Active));
}
