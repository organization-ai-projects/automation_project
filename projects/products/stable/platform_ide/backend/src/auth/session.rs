// projects/products/stable/platform_ide/backend/src/auth/session.rs

/// An authenticated IDE session.
///
/// The bearer token is stored in memory only and is never persisted to disk
/// or emitted to logs.
#[derive(Clone)]
pub struct Session {
    /// The bearer token issued by the platform.
    token: String,
    /// The subject (user identifier) this session belongs to.
    pub subject: String,
}

impl Session {
    /// Creates a new session from a bearer token and its subject.
    pub fn new(token: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            subject: subject.into(),
        }
    }

    /// Returns the bearer token string for use in HTTP `Authorization` headers.
    pub fn bearer_token(&self) -> &str {
        &self.token
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Token is intentionally redacted from debug output.
        f.debug_struct("Session")
            .field("subject", &self.subject)
            .field("token", &"[REDACTED]")
            .finish()
    }
}
