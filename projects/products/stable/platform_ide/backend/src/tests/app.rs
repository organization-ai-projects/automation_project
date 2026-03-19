//! projects/products/stable/platform_ide/backend/src/tests/app.rs
use crate::app::IdeApp;
use crate::auth::Session;

#[test]
fn app_new_sets_default_state() {
    let session = Session::new("token", "subject");
    let app = IdeApp::new("http://127.0.0.1:8080", session);

    assert!(app.active_issue.is_none());
    assert!(app.manifest.is_none());
    assert!(!app.show_offline_controls());
}
