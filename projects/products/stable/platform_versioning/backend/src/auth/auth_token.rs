// projects/products/stable/platform_versioning/backend/src/auth/auth_token.rs
use serde::{Deserialize, Serialize};

/// An opaque bearer token provided by the client.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuthToken(String);

impl AuthToken {
    /// Creates an `AuthToken` from the raw bearer string.
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    /// Returns the raw token string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
