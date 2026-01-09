// projects/libraries/security/src/auth.rs
use common::common_id::is_valid_id;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserId(u64);

impl UserId {
    /// Crée un UserId validé
    pub fn new(id: u64) -> Result<Self, crate::TokenError> {
        if !is_valid_id(id) {
            return Err(crate::TokenError::InvalidUserIdValue);
        }
        Ok(Self(id))
    }

    /// Retourne la valeur brute
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Vérifie si l'ID est valide
    pub fn is_valid(&self) -> bool {
        is_valid_id(self.0)
    }
}

// TryFrom pour conversion sûre depuis u64
impl TryFrom<u64> for UserId {
    type Error = crate::TokenError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        Self::new(id)
    }
}

// Implémentation du trait FromStr pour UserId
impl FromStr for UserId {
    type Err = crate::TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .trim()
            .parse::<u64>()
            .map_err(|_| crate::TokenError::InvalidUserIdFormat)?;
        Self::new(id)
    }
}

// From<&str> pour commodité (utilise 0 en cas d'échec - à utiliser avec précaution)
impl From<&str> for UserId {
    fn from(id: &str) -> Self {
        id.parse().unwrap_or(UserId(0))
    }
}

// Conversion vers String
impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.0.to_string()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for UserId {
    fn eq(&self, other: &&str) -> bool {
        self.0.to_string() == *other
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
        let user_id = UserId::new(1).unwrap();
        assert_eq!(user_id.value(), 1);
    }

    #[test]
    fn test_user_id_new_invalid() {
        assert!(UserId::new(0).is_err());
    }

    #[test]
    fn test_user_id_try_from_u64() {
        let user_id = UserId::try_from(1u64).unwrap();
        assert_eq!(user_id.value(), 1);

        assert!(UserId::try_from(0u64).is_err());
    }

    #[test]
    fn test_user_id_try_from_str() {
        let user_id = "1".parse::<UserId>().unwrap();
        assert_eq!(user_id.value(), 1);

        assert!("0".parse::<UserId>().is_err());
        assert!("invalid".parse::<UserId>().is_err());
        assert!("".parse::<UserId>().is_err());
    }

    #[test]
    fn test_user_id_from_str_fallback() {
        // From<&str> utilise 0 en cas d'échec
        let user_id = UserId::from("invalid");
        assert_eq!(user_id.value(), 0);
    }

    #[test]
    fn test_user_id_equality() {
        let id1 = UserId::new(1).unwrap();
        let id2 = UserId::new(1).unwrap();
        let id3 = UserId::new(2).unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_eq!(id1, "1");
    }

    #[test]
    fn test_user_id_display() {
        let user_id = UserId::new(42).unwrap();
        assert_eq!(format!("{}", user_id), "42");
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new("1".parse::<UserId>().unwrap(), Role::User, 3_600_000).unwrap();
        assert_eq!(token.user_id.value(), 1);
        assert_eq!(token.role, Role::User);
        assert!(!token.is_expired());
    }

    #[test]
    fn test_token_not_expired() {
        let token = Token::new("1".parse::<UserId>().unwrap(), Role::User, 3_600_000).unwrap();
        assert!(!token.is_expired());
        assert!(token.validate_token().is_ok());
        assert!(token.time_until_expiry_ms() > 0);
    }

    #[test]
    fn test_token_expired() {
        let token = Token::new("1".parse::<UserId>().unwrap(), Role::User, 1).unwrap();
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
            "1".parse::<UserId>().unwrap(),
            Role::Admin,
            3_600_000,
            "session_xyz".to_string(),
        )
        .unwrap();
        assert_eq!(token.session_id.as_deref(), Some("session_xyz"));
    }

    #[test]
    fn test_token_age() {
        let token = Token::new("1".parse::<UserId>().unwrap(), Role::User, 3_600_000).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        assert!(token.age_ms() > 0);
    }

    #[test]
    fn test_validate_user_id() {
        assert!(validate_user_id("1"));
        assert!(validate_user_id("  1  "));
        assert!(!validate_user_id(""));
        assert!(!validate_user_id("   "));
        assert!(!validate_user_id("0"));
        assert!(!validate_user_id("invalid"));
    }
}
