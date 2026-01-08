use common::common_id::is_valid_id;

use crate::auth_error::AuthError;
use crate::token::Token;

/// Valide un token (structure + expiration)
pub fn validate_token(token: &Token) -> Result<(), AuthError> {
    if !token.validate() {
        return Err(AuthError::InvalidToken);
    }

    if token.is_expired() {
        return Err(AuthError::TokenExpired);
    }

    Ok(())
}

pub fn validate_user_id(user_id: &str) -> bool {
    let trimmed = user_id.trim();
    if trimmed.is_empty() {
        return false;
    }

    if let Ok(id) = trimmed.parse::<u64>() {
        is_valid_id(id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::validate_user_id, role::Role};
    use crate::{auth::validate_token as validate_auth_token, AuthError, Token};

    #[test]
    fn test_token_creation() {
        // Token::new attend maintenant duration_ms + user_id numérique valide
        let token = Token::new("1".to_string(), Role::User, 3_600_000).unwrap();

        assert_eq!(token.user_id, "1");
        assert_eq!(token.role, Role::User);
        assert!(!token.is_expired());
        assert!(token.validate());
    }

    #[test]
    fn test_token_not_expired() {
        let token = Token::new("1".to_string(), Role::User, 3_600_000).unwrap();

        assert!(!token.is_expired());
        assert!(validate_auth_token(&token).is_ok());

        // en ms, toujours >= 0
        assert!(token.time_until_expiry_ms() > 0);
    }

    #[test]
    fn test_token_expired() {
        // durée 1ms, puis on attend un peu
        let token = Token::new("1".to_string(), Role::User, 1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));

        assert!(token.is_expired());
        assert!(matches!(
            validate_auth_token(&token),
            Err(AuthError::TokenExpired) | Err(AuthError::InvalidToken)
        ));
    }

    #[test]
    fn test_token_with_session() {
        let token = Token::new_with_session(
            "1".to_string(),
            Role::Admin,
            3_600_000,
            "session_xyz".to_string(),
        )
        .unwrap();

        assert_eq!(token.session_id.as_deref(), Some("session_xyz"));
    }

    #[test]
    fn test_token_age() {
        let token = Token::new("1".to_string(), Role::User, 3_600_000).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));

        assert!(token.age_ms() > 0);
    }

    #[test]
    fn test_token_renewal() {
        let mut token = Token::new("1".to_string(), Role::User, 10).unwrap();
        let initial_expiry = token.expires_at_ms;

        token.renew(1000).unwrap();

        // renew peut renouveler depuis expires_at si pas expiré
        assert_eq!(token.expires_at_ms, initial_expiry + 1000);
    }

    #[test]
    fn test_validate_user_id() {
        // IMPORTANT: ton validate_user_id vérifie un id numérique + is_valid_id
        assert!(validate_user_id("1"));
        assert!(validate_user_id("  1  "));

        assert!(!validate_user_id(""));
        assert!(!validate_user_id("   "));
        assert!(!validate_user_id("valid_user")); // <- avant ton test était incohérent avec is_valid_id
    }
}
