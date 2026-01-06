// security/src/lib.rs
pub mod auth;
pub mod permissions;
pub mod role;

pub use auth::{validate_token, Token, AuthError, token_duration};
pub use permissions::{
    has_permission,
    has_all_permissions,
    has_any_permission,
    check_permission,
    check_all_permissions,
    check_token_permission,
    check_token_all_permissions,
    filter_allowed_permissions,
    missing_permissions,
};
pub use role::{Role, Permission};

pub fn init() {
    println!("Initializing security library...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        // Créer un token
        let token = Token::new("alice".to_string(), Role::User, token_duration::ONE_HOUR);

        // Vérifier qu'il est valide
        assert!(validate_token(&token).is_ok());

        // Vérifier les permissions
        assert!(check_token_permission(&token, Permission::Write).is_ok());
        assert!(check_token_permission(&token, Permission::Admin).is_err());

        // Vérifier les permissions du rôle directement
        assert!(has_permission(token.role(), Permission::Read));
        assert!(!has_permission(token.role(), Permission::Delete));
    }
}