use crate::state::app_state::AppState;

#[test]
fn default_state() {
    let state = AppState::new();
    assert!(state.status().is_empty());
    assert!(state.report().is_none());
    assert!(state.scenario_path().is_none());
}

#[test]
fn set_and_get_status() {
    let mut state = AppState::new();
    state.set_status("idle".into());
    assert_eq!(state.status(), "idle");
}

#[test]
fn set_and_get_report() {
    let mut state = AppState::new();
    state.set_report(r#"{"run_hash":"abc"}"#.into());
    assert_eq!(state.report(), Some(r#"{"run_hash":"abc"}"#));
}
