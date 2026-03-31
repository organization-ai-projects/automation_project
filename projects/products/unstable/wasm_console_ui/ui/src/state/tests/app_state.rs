use crate::state::app_state::AppState;

#[test]
fn new_state_defaults() {
    let state = AppState::new();
    assert!(state.active_panel_id.is_none());
    assert!(state.panel_titles.is_empty());
    assert!(state.status_message.is_none());
    assert!(state.error_message.is_none());
    assert!(state.log_content.is_none());
    assert!(state.report_content.is_none());
    assert!(state.graph_content.is_none());
    assert!(state.snapshot_json.is_none());
}

#[test]
fn has_error_false_by_default() {
    let state = AppState::new();
    assert!(!state.has_error());
}

#[test]
fn has_error_true_when_set() {
    let mut state = AppState::new();
    state.error_message = Some("error".to_string());
    assert!(state.has_error());
}

#[test]
fn has_status_false_by_default() {
    let state = AppState::new();
    assert!(!state.has_status());
}

#[test]
fn default_equals_new() {
    assert_eq!(AppState::new(), AppState::default());
}
