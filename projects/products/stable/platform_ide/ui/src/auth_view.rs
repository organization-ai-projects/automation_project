// projects/products/stable/platform_ide/ui/src/auth_view.rs
use serde::{Deserialize, Serialize};

/// The authentication view state.
///
/// Presents a login prompt to the user and stores the resulting session token.
/// The token field is intentionally `Option<String>` so that logout clears it
/// from display state.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthView {
    /// Whether the user is currently authenticated.
    pub authenticated: bool,
    /// The bearer token obtained after successful authentication.
    pub token: Option<String>,
    /// The subject (username) displayed after login.
    pub subject: Option<String>,
}

impl AuthView {
    /// Records a successful login.
    pub fn login(&mut self, token: String, subject: String) {
        self.authenticated = true;
        self.token = Some(token);
        self.subject = Some(subject);
    }

    /// Clears the authentication state.
    pub fn logout(&mut self) {
        self.authenticated = false;
        self.token = None;
        self.subject = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
