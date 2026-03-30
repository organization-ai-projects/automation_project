use crate::app::app_state::AppState;

#[test]
fn new_state_has_no_active_panel() {
    let state = AppState::new();
    assert!(state.active_panel.is_none());
}

#[test]
fn new_state_has_empty_panels() {
    let state = AppState::new();
    assert!(state.panels.is_empty());
}

#[test]
fn new_state_has_no_status() {
    let state = AppState::new();
    assert!(state.status_message.is_none());
    assert!(state.error_message.is_none());
}

#[test]
fn default_equals_new() {
    let a = AppState::new();
    let b = AppState::default();
    assert_eq!(a, b);
}
