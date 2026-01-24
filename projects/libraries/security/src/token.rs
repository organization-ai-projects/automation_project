// projects/libraries/security/src/token.rs
use crate::auth::UserId;
use crate::{auth_error, role::Role};
use common::custom_uuid::Id128;
use common_time::timestamp_utils;
use serde::{Deserialize, Serialize};

/// Verified token (internal struct convenient for the app).
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
    /// Checks if the token is expired
    pub fn is_expired(&self) -> bool {
        self.is_expired_with_grace(0)
    }

    /// Checks if the token is expired with an optional grace period
    pub fn is_expired_with_grace(&self, grace_ms: u64) -> bool {
        let now = timestamp_utils::current_timestamp_ms();
        self.expires_at_ms.saturating_add(grace_ms) <= now
    }

    /// Returns the remaining time until expiration in milliseconds
    pub fn time_until_expiry_ms(&self) -> i64 {
        let now = timestamp_utils::current_timestamp_ms() as i64;
        self.expires_at_ms as i64 - now
    }

    /// Returns the age of the token in milliseconds
    pub fn age_ms(&self) -> u64 {
        let now = timestamp_utils::current_timestamp_ms();
        now.saturating_sub(self.issued_at_ms)
    }

    /// Validates a token (structure + expiration)
    pub fn validate_token(&self) -> Result<(), auth_error::AuthError> {
        let zero_id = Id128::from_bytes_unchecked([0u8; 16]);
        if self.user_id.value() == zero_id {
            return Err(auth_error::AuthError::InvalidToken);
        }
        if self.is_expired() {
            return Err(auth_error::AuthError::TokenExpired);
        }
        Ok(())
    }

    #[cfg(test)]
    /// Creates a new token (for tests only)
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
    /// Creates a new token with a session (for tests only)
    pub fn new_with_session(
        user_id: UserId,
        role: Role,
        duration_ms: u64,
        session_id: String,
    ) -> Result<Self, String> {
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
            session_id: Some(session_id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::custom_uuid::Id128; // Fixed missing semicolon

    #[test]
    fn test_is_expired() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 1).expect("token");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(token.is_expired());
    }

    #[test]
    fn test_is_expired_with_grace() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 50).expect("token");
        std::thread::sleep(std::time::Duration::from_millis(60));

        // Expired without grace
        assert!(token.is_expired());

        // Not expired with 100ms grace
        assert!(!token.is_expired_with_grace(100));
    }

    #[test]
    fn test_time_until_expiry() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 5000).expect("token");
        let remaining = token.time_until_expiry_ms();
        assert!(remaining > 4500 && remaining <= 5000);
    }

    #[test]
    fn test_validate_token() {
        let id_valid = Id128::from_bytes_unchecked([1u8; 16]);
        let valid = Token::new(UserId::from(id_valid), Role::User, 5000).expect("token");
        assert!(valid.validate_token().is_ok());

        let id_expired = Id128::from_bytes_unchecked([2u8; 16]);
        let expired = Token::new(UserId::from(id_expired), Role::User, 1).expect("token");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(matches!(
            expired.validate_token(),
            Err(auth_error::AuthError::TokenExpired)
        ));
    }

    #[test]
    fn test_token_creation() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 3600000).expect("token");
        assert_eq!(token.user_id.value(), id);
    }

    #[test]
    fn test_token_with_session() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new_with_session(
            UserId::from(id),
            Role::Admin,
            3600000,
            "session_xyz".to_string(),
        )
        .expect("token");
        assert_eq!(token.session_id.as_deref(), Some("session_xyz"));
    }

    #[test]
    fn test_token_expired() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 1).expect("token");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(token.is_expired());
    }

    #[test]
    fn test_validate_token_valid() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 5000).expect("token");
        assert!(token.validate_token().is_ok());
    }

    #[test]
    fn test_validate_token_invalid() {
        let id = Id128::from_bytes_unchecked([0u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 5000).expect("token");
        assert!(token.validate_token().is_err());
    }

    #[test]
    fn test_token_creation_with_id128() {
        let id = Id128::from_bytes_unchecked([123u8; 16]);
        let token = Token::new(UserId::from(id), Role::User, 5000).expect("token");
        assert_eq!(token.user_id.value(), id);
    }
}
