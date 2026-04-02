use crate::app::App;

#[test]
fn app_new_default_state() {
    let app = App::new();
    assert!(app.state().active_panel_id.is_none());
    assert!(app.state().panel_titles.is_empty());
}

#[test]
fn app_default_equals_new() {
    let a = App::new();
    let b = App::default();
    assert_eq!(a.state(), b.state());
}

#[test]
fn app_update_state() {
    let mut app = App::new();
    let mut new_state = crate::state::app_state::AppState::new();
    new_state.status_message = Some("updated".to_string());
    app.update_state(new_state.clone());
    assert_eq!(app.state().status_message, Some("updated".to_string()));
}
