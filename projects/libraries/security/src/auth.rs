use crate::AuthError;
// security/src/auth.rs
use crate::token::Token;
use common::utils::is_valid_name;

/// Valide un token
pub fn validate_token(token: &Token) -> Result<(), AuthError> {
    if token.is_expired() {
        return Err(AuthError::TokenExpired);
    }

    Ok(())
}

pub fn validate_user_id(user_id: &str) -> bool {
    is_valid_name(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::role::Role;
    use common::utils::time_durations;

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
        assert!(matches!(
            validate_token(&token),
            Err(AuthError::TokenExpired)
        ));
        assert!(token.time_until_expiry().is_none());
    }

    #[test]
    fn test_token_with_session() {
        let token = Token::new_with_session(
            "user123".to_string(),
            Role::Admin,
            3600,
            "session_xyz".to_string(),
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
        assert_eq!(time_durations::ONE_HOUR, 3600);
        assert_eq!(time_durations::ONE_DAY, 86400);
    }

    #[test]
    fn test_validate_user_id() {
        assert!(validate_user_id("valid_user"));
        assert!(!validate_user_id(""));
        assert!(!validate_user_id("   "));
    }
}
