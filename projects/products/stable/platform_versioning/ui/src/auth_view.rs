// projects/products/stable/platform_versioning/ui/src/auth_view.rs
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
