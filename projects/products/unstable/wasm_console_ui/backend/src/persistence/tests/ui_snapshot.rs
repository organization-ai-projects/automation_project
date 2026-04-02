use crate::persistence::ui_snapshot::UiSnapshot;

#[test]
fn snapshot_version_is_one() {
    let snapshot = UiSnapshot::new("{}".to_string(), "abc".to_string());
    assert_eq!(snapshot.version, 1);
}

#[test]
fn snapshot_preserves_state_json() {
    let state_json =
        r#"{"active_panel":null,"panels":[],"status_message":null,"error_message":null}"#
            .to_string();
    let snapshot = UiSnapshot::new(state_json.clone(), "abc".to_string());
    assert_eq!(snapshot.state_json, state_json);
}
