// projects/libraries/security/src/token.rs
use crate::auth::UserId;
use crate::{auth_error, role::Role};
use common_time::timestamp_utils;
use serde::{Deserialize, Serialize};

/// Token vérifié (struct interne pratique pour l'app).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub value: String,
    pub user_id: UserId,
    pub role: Role,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub session_id: Option<String>,
}

impl Token {
    /// Vérifie si le token est expiré
    pub fn is_expired(&self) -> bool {
        self.is_expired_with_grace(0)
    }

    /// Vérifie si le token est expiré avec un délai de grâce optionnel
    pub fn is_expired_with_grace(&self, grace_ms: u64) -> bool {
        let now = timestamp_utils::current_timestamp_ms();
        self.expires_at_ms.saturating_add(grace_ms) <= now
    }

    /// Retourne le temps restant avant l'expiration en millisecondes
    pub fn time_until_expiry_ms(&self) -> i64 {
        let now = timestamp_utils::current_timestamp_ms() as i64;
        self.expires_at_ms as i64 - now
    }

    /// Retourne l'âge du token en millisecondes
    pub fn age_ms(&self) -> u64 {
        let now = timestamp_utils::current_timestamp_ms();
        now.saturating_sub(self.issued_at_ms)
    }

    /// Valide un token (structure + expiration)
    pub fn validate_token(&self) -> Result<(), auth_error::AuthError> {
        if self.user_id.value() == 0 {
            return Err(auth_error::AuthError::InvalidToken);
        }
        if self.is_expired() {
            return Err(auth_error::AuthError::TokenExpired);
        }
        Ok(())
    }

    #[cfg(test)]
    /// Crée un nouveau token (uniquement pour les tests)
    pub fn new(user_id: UserId, role: Role, duration_ms: u64) -> Result<Self, String> {
        if duration_ms == 0 {
            return Err("Duration must be greater than 0".to_string());
        }
        let issued_at_ms = timestamp_utils::current_timestamp_ms();
        let expires_at_ms = issued_at_ms
            .checked_add(duration_ms)
            .ok_or("Timestamp overflow")?;

        Ok(Self {
            value: "test_value".to_string(),
            user_id,
            role,
            issued_at_ms,
            expires_at_ms,
            session_id: None,
        })
    }

    #[cfg(test)]
    /// Crée un nouveau token avec une session (uniquement pour les tests)
    pub fn new_with_session(
        user_id: UserId,
        role: Role,
        duration_ms: u64,
        session_id: String,
    ) -> Result<Self, String> {
        let mut token = Self::new(user_id, role, duration_ms)?;
        token.session_id = Some(session_id);
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_expired() {
        let token = Token::new(UserId::from("123"), Role::User, 1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(token.is_expired());
    }

    #[test]
    fn test_is_expired_with_grace() {
        let token = Token::new(UserId::from("123"), Role::User, 50).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(60));

        // Expiré sans grâce
        assert!(token.is_expired());

        // Pas expiré avec 100ms de grâce
        assert!(!token.is_expired_with_grace(100));
    }

    #[test]
    fn test_time_until_expiry() {
        let token = Token::new(UserId::from("123"), Role::User, 5000).unwrap();
        let remaining = token.time_until_expiry_ms();
        assert!(remaining > 4500 && remaining <= 5000);
    }

    #[test]
    fn test_validate_token() {
        let valid = Token::new(UserId::from("123"), Role::User, 5000).unwrap();
        assert!(valid.validate_token().is_ok());

        let expired = Token::new(UserId::from("123"), Role::User, 1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(matches!(
            expired.validate_token(),
            Err(auth_error::AuthError::TokenExpired)
        ));
    }
}