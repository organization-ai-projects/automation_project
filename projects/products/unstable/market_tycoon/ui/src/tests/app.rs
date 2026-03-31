use crate::app::App;

#[test]
fn new_app_has_empty_state() {
    let app = App::new();
    assert!(app.state().status().is_empty());
}

#[test]
fn update_status() {
    let mut app = App::new();
    app.update_status("Running".into());
    assert_eq!(app.state().status(), "Running");
}
