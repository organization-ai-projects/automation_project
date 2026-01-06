// security/src/auth.rs
use crate::role::Role;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Unauthorized: insufficient permissions")]
    Unauthorized,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Session error: {0}")]
    SessionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Identifiant unique de l'utilisateur
    pub user_id: String,

    /// Rôle de l'utilisateur
    pub role: Role,

    /// Timestamp de création (secondes depuis UNIX_EPOCH)
    pub issued_at: u64,

    /// Timestamp d'expiration (secondes depuis UNIX_EPOCH)
    pub expires_at: u64,

    /// Session ID pour révocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl Token {
    /// Crée un nouveau token
    pub fn new(user_id: String, role: Role, duration_secs: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            user_id,
            role,
            issued_at: now,
            expires_at: now + duration_secs,
            session_id: None,
        }
    }

    /// Crée un token avec session ID
    pub fn new_with_session(user_id: String, role: Role, duration_secs: u64, session_id: String) -> Self {
        let mut token = Self::new(user_id, role, duration_secs);
        token.session_id = Some(session_id);
        token
    }

    /// Vérifie si le token est expiré
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now > self.expires_at
    }

    /// Retourne le temps restant avant expiration (en secondes)
    pub fn time_until_expiry(&self) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if self.expires_at > now {
            Some(self.expires_at - now)
        } else {
            None
        }
    }

    /// Retourne le rôle du token
    pub fn role(&self) -> &Role {
        &self.role
    }

    /// Retourne l'âge du token (en secondes)
    pub fn age(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now.saturating_sub(self.issued_at)
    }

    /// Renouvelle le token (extend expiration)
    pub fn renew(&mut self, additional_duration_secs: u64) {
        self.expires_at += additional_duration_secs;
    }
}

/// Valide un token
pub fn validate_token(token: &Token) -> Result<(), AuthError> {
    if token.is_expired() {
        return Err(AuthError::TokenExpired);
    }

    Ok(())
}

/// Durées standard pour les tokens
pub mod token_duration {
    pub const FIFTEEN_MINUTES: u64 = 15 * 60;
    pub const ONE_HOUR: u64 = 60 * 60;
    pub const ONE_DAY: u64 = 24 * 60 * 60;
    pub const ONE_WEEK: u64 = 7 * 24 * 60 * 60;
    pub const ONE_MONTH: u64 = 30 * 24 * 60 * 60;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new("user123".to_string(), Role::User, 3600);

        assert_eq!(token.user_id, "user123");
        assert_eq!(token.role, Role::User);
        assert!(!token.is_expired());
    }

    #[test]
    fn test_token_not_expired() {
        let token = Token::new("user123".to_string(), Role::User, 3600);
        assert!(!token.is_expired());
        assert!(validate_token(&token).is_ok());
        assert!(token.time_until_expiry().is_some());
    }

    #[test]
    fn test_token_expired() {
        let token = Token::new("user123".to_string(), Role::User, 0);
        std::thread::sleep(std::time::Duration::from_millis(10));

        assert!(token.is_expired());
        assert!(matches!(validate_token(&token), Err(AuthError::TokenExpired)));
        assert!(token.time_until_expiry().is_none());
    }

    #[test]
    fn test_token_with_session() {
        let token = Token::new_with_session(
            "user123".to_string(),
            Role::Admin,
            3600,
            "session_xyz".to_string()
        );

        assert_eq!(token.session_id, Some("session_xyz".to_string()));
    }

    #[test]
    fn test_token_age() {
        let token = Token::new("user123".to_string(), Role::User, 3600);
        std::thread::sleep(std::time::Duration::from_millis(100));

        assert!(token.age() > 0);
    }

    #[test]
    fn test_token_renewal() {
        let mut token = Token::new("user123".to_string(), Role::User, 100);
        let initial_expiry = token.expires_at;

        token.renew(1000);

        assert_eq!(token.expires_at, initial_expiry + 1000);
    }

    #[test]
    fn test_token_durations() {
        assert_eq!(token_duration::ONE_HOUR, 3600);
        assert_eq!(token_duration::ONE_DAY, 86400);
    }
}