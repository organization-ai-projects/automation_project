// projects/products/unstable/platform_versioning/ui/src/auth_view.rs
use serde::{Deserialize, Serialize};

/// The authentication view state.
///
/// Presents a login prompt to the user and stores the resulting token.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthView {
    /// Whether the user is currently authenticated.
    pub authenticated: bool,
    /// The bearer token obtained after successful authentication.
    pub token: Option<String>,
}

impl AuthView {
    /// Records a successful login with the given token.
    pub fn login(&mut self, token: String) {
        self.authenticated = true;
        self.token = Some(token);
    }

    /// Clears the authentication state.
    pub fn logout(&mut self) {
        self.authenticated = false;
        self.token = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_sets_authenticated() {
        let mut view = AuthView::default();
        view.login("my-token".to_string());
        assert!(view.authenticated);
        assert_eq!(view.token.as_deref(), Some("my-token"));
    }

    #[test]
    fn logout_clears_state() {
        let mut view = AuthView::default();
        view.login("my-token".to_string());
        view.logout();
        assert!(!view.authenticated);
        assert!(view.token.is_none());
    }
}
