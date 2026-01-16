// projects/libraries/security/src/auth.rs
use common::common_id::CommonID;
use common::custom_uuid::Id128;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::TokenError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserId(Id128);

impl UserId {
    /// Crée un UserId validé
    pub fn new(id: Id128) -> Result<Self, TokenError> {
        if !CommonID::is_valid(id) {
            return Err(TokenError::InvalidUserIdValue);
        }
        Ok(Self(id))
    }

    /// Retourne l'identifiant sous forme d'Id128
    pub fn value(&self) -> Id128 {
        self.0
    }
}

// TryFrom pour conversion sûre depuis u64
impl TryFrom<u64> for UserId {
    type Error = TokenError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        Self::new(Id128::new(id as u16, None, None))
    }
}

// Implémentation du trait FromStr pour UserId
impl FromStr for UserId {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .trim()
            .parse::<u128>()
            .map_err(|_| TokenError::InvalidUserIdFormat)?;
        let id128 = Id128::from_bytes_unchecked(id.to_be_bytes());
        Self::new(id128)
    }
}

// Conversion vers String
impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.0.to_string()
    }
}

// Implémentation de From<Id128> pour UserId
impl From<Id128> for UserId {
    fn from(id: Id128) -> Self {
        UserId(id)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Fonction utilitaire pour valider un user_id string
pub fn validate_user_id(user_id: &str) -> bool {
    user_id.parse::<UserId>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::role::Role;
    use crate::{AuthError, Token};

    #[test]
    fn test_user_id_new_valid() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let user_id = UserId::new(id).unwrap();
        assert_eq!(user_id.value(), id);
    }

    #[test]
    fn test_user_id_new_invalid() {
        let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);
        assert!(UserId::new(invalid_id).is_err());
    }

    #[test]
    fn test_user_id_equality() {
        let id1 = Id128::from_bytes_unchecked([1u8; 16]);
        let id2 = Id128::from_bytes_unchecked([1u8; 16]);
        let id3 = Id128::from_bytes_unchecked([2u8; 16]);

        let user_id1 = UserId::new(id1).unwrap();
        let user_id2 = UserId::new(id2).unwrap();
        let user_id3 = UserId::new(id3).unwrap();

        assert_eq!(user_id1, user_id2);
        assert_ne!(user_id1, user_id3);
    }

    #[test]
    fn test_user_id_display() {
        let id = Id128::from_bytes_unchecked([42u8; 16]);
        let user_id = UserId::new(id).unwrap();
        assert_eq!(format!("{}", user_id), id.to_string());
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new(
            UserId::new(Id128::from_bytes_unchecked([1u8; 16])).unwrap(),
            Role::User,
            3_600_000,
        )
        .unwrap();
        assert_eq!(
            token.user_id.value(),
            Id128::from_bytes_unchecked([1u8; 16])
        );
        assert_eq!(token.role, Role::User);
        assert!(!token.is_expired());
    }

    #[test]
    fn test_token_not_expired() {
        let token = Token::new(
            UserId::new(Id128::from_bytes_unchecked([1u8; 16])).unwrap(),
            Role::User,
            3_600_000,
        )
        .unwrap();
        assert!(!token.is_expired());
        assert!(token.validate_token().is_ok());
        assert!(token.time_until_expiry_ms() > 0);
    }

    #[test]
    fn test_token_expired() {
        let token = Token::new(
            UserId::new(Id128::from_bytes_unchecked([1u8; 16])).unwrap(),
            Role::User,
            1,
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(token.is_expired());
        assert!(matches!(
            token.validate_token(),
            Err(AuthError::TokenExpired)
        ));
    }

    #[test]
    fn test_token_with_session() {
        let token = Token::new_with_session(
            UserId::new(Id128::from_bytes_unchecked([1u8; 16])).unwrap(),
            Role::Admin,
            3_600_000,
            "session_xyz".to_string(),
        )
        .unwrap();
        assert_eq!(token.session_id.as_deref(), Some("session_xyz"));
    }

    #[test]
    fn test_token_age() {
        let token = Token::new(
            UserId::new(Id128::from_bytes_unchecked([1u8; 16])).unwrap(),
            Role::User,
            3_600_000,
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        assert!(token.age_ms() > 0);
    }

    #[test]
    fn test_validate_user_id() {
        // Vérifie que "0" est invalide
        let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);
        assert!(!CommonID::is_valid(invalid_id));
    }

    #[test]
    fn test_user_id_new_with_id128() {
        // Teste directement avec Id128
        let valid_id = Id128::from_bytes_unchecked([1u8; 16]);
        let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);

        assert!(CommonID::is_valid(valid_id));
        assert!(!CommonID::is_valid(invalid_id));
    }
}
