use crate::components::snapshot_controls::SnapshotControls;

#[test]
fn snapshot_controls_no_snapshot() {
    let controls = SnapshotControls::new(None);
    assert!(!controls.has_snapshot());
}

#[test]
fn snapshot_controls_with_snapshot() {
    let controls = SnapshotControls::new(Some("{}".to_string()));
    assert!(controls.has_snapshot());
}
