use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::persistence::snapshot_codec::SnapshotCodec;

#[test]
fn export_import_roundtrip() {
    let state = AppState::new();
    let exported = SnapshotCodec::export(&state).unwrap();
    let imported = SnapshotCodec::import(&exported).unwrap();
    assert_eq!(state, imported);
}

#[test]
fn export_import_with_panels() {
    let mut state = AppState::new();
    state = Reducer::reduce(
        &state,
        &Action::LoadLogFile {
            path: "test.json".to_string(),
        },
    );
    state = Reducer::reduce(
        &state,
        &Action::LoadReportFile {
            path: "report.json".to_string(),
        },
    );
    let exported = SnapshotCodec::export(&state).unwrap();
    let imported = SnapshotCodec::import(&exported).unwrap();
    assert_eq!(state, imported);
}

#[test]
fn export_is_deterministic() {
    let state = AppState::new();
    let a = SnapshotCodec::export(&state).unwrap();
    let b = SnapshotCodec::export(&state).unwrap();
    // Compare deserialized snapshots since outer JSON field order may vary
    let snap_a: crate::persistence::ui_snapshot::UiSnapshot = common_json::from_str(&a).unwrap();
    let snap_b: crate::persistence::ui_snapshot::UiSnapshot = common_json::from_str(&b).unwrap();
    assert_eq!(snap_a, snap_b);
}

#[test]
fn import_detects_corrupted_checksum() {
    let state = AppState::new();
    let exported = SnapshotCodec::export(&state).unwrap();
    let corrupted = exported.replace(
        &exported[exported.len() - 10..exported.len() - 2],
        "XXXXXXXX",
    );
    let result = SnapshotCodec::import(&corrupted);
    assert!(result.is_err());
}

#[test]
fn repeated_export_import_is_stable() {
    let state = AppState::new();
    let exported1 = SnapshotCodec::export(&state).unwrap();
    let imported1 = SnapshotCodec::import(&exported1).unwrap();
    let exported2 = SnapshotCodec::export(&imported1).unwrap();
    let imported2 = SnapshotCodec::import(&exported2).unwrap();
    assert_eq!(imported1, imported2);
    // Compare deserialized snapshots since outer JSON field order may vary
    let snap1: crate::persistence::ui_snapshot::UiSnapshot =
        common_json::from_str(&exported1).unwrap();
    let snap2: crate::persistence::ui_snapshot::UiSnapshot =
        common_json::from_str(&exported2).unwrap();
    assert_eq!(snap1, snap2);
}
