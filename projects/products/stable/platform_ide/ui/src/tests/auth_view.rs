use crate::auth_view::AuthView;

#[test]
fn login_sets_authenticated() {
    let mut view = AuthView::default();
    view.login("my-token".to_string(), "alice".to_string());
    assert!(view.authenticated);
    assert_eq!(view.token.as_deref(), Some("my-token"));
    assert_eq!(view.subject.as_deref(), Some("alice"));
}

#[test]
fn logout_clears_state() {
    let mut view = AuthView::default();
    view.login("my-token".to_string(), "alice".to_string());
    view.logout();
    assert!(!view.authenticated);
    assert!(view.token.is_none());
    assert!(view.subject.is_none());
}
