// security/src/lib.rs
pub mod auth;
pub mod auth_error;
pub mod permissions;
pub mod role;
pub mod token;

pub use crate::token::Token;
pub use auth::validate_token;
pub use auth_error::AuthError;
pub use permissions::{
    check_all_permissions, check_permission, check_token_all_permissions, check_token_permission,
    filter_allowed_permissions, has_all_permissions, has_any_permission, has_permission,
    missing_permissions,
};
pub use role::{Permission, Role};

pub fn init() {
    println!("Initializing security library...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::time_durations;

    #[test]
    fn test_integration() {
        // Créer un token
        let token = Token::new("alice".to_string(), Role::User, time_durations::ONE_HOUR);

        // Vérifier qu'il est valide
        assert!(validate_token(&token).is_ok());

        // Vérifier les permissions
        assert!(check_token_permission(&token, Permission::Write).is_ok());
        assert!(check_token_permission(&token, Permission::Admin).is_err());

        // Vérifier les permissions du rôle directement
        assert!(has_permission(&token.role, Permission::Read));
        assert!(!has_permission(&token.role, Permission::Delete));
    }
}
