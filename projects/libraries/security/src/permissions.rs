use crate::AuthError;
// security/src/permissions.rs
use crate::role::{Permission, Role};
use crate::token::Token;

/// Vérifie si un rôle a une permission spécifique
pub fn has_permission(role: &Role, required_permission: Permission) -> bool {
    role.has_permission(required_permission)
}

/// Vérifie si un rôle a plusieurs permissions à la fois
pub fn has_all_permissions(role: &Role, required_permissions: &[Permission]) -> bool {
    required_permissions
        .iter()
        .all(|&perm| role.has_permission(perm))
}

/// Vérifie si un rôle a au moins une des permissions listées
pub fn has_any_permission(role: &Role, required_permissions: &[Permission]) -> bool {
    required_permissions
        .iter()
        .any(|&perm| role.has_permission(perm))
}

/// Vérifie si un rôle a une permission, retourne une erreur sinon
pub fn check_permission(role: &Role, required_permission: Permission) -> Result<(), AuthError> {
    if has_permission(role, required_permission) {
        Ok(())
    } else {
        Err(AuthError::Unauthorized)
    }
}

/// Vérifie si un rôle a toutes les permissions requises
pub fn check_all_permissions(
    role: &Role,
    required_permissions: &[Permission],
) -> Result<(), AuthError> {
    if has_all_permissions(role, required_permissions) {
        Ok(())
    } else {
        Err(AuthError::Unauthorized)
    }
}

/// Vérifie si un token valide a une permission
pub fn check_token_permission(
    token: &Token,
    required_permission: Permission,
) -> Result<(), AuthError> {
    crate::auth::validate_token(token)?;
    check_permission(&token.role, required_permission)
}

/// Vérifie si un token valide a toutes les permissions requises
pub fn check_token_all_permissions(
    token: &Token,
    required_permissions: &[Permission],
) -> Result<(), AuthError> {
    crate::auth::validate_token(token)?;
    check_all_permissions(&token.role, required_permissions)
}

/// Filtre une liste de permissions en ne gardant que celles qu'un rôle possède
pub fn filter_allowed_permissions(role: &Role, permissions: &[Permission]) -> Vec<Permission> {
    permissions
        .iter()
        .copied()
        .filter(|&perm| role.has_permission(perm))
        .collect()
}

/// Retourne les permissions manquantes pour un rôle
pub fn missing_permissions(role: &Role, required_permissions: &[Permission]) -> Vec<Permission> {
    required_permissions
        .iter()
        .copied()
        .filter(|&perm| !role.has_permission(perm))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_permission() {
        assert!(has_permission(&Role::Admin, Permission::Admin));
        assert!(has_permission(&Role::User, Permission::Write));
        assert!(!has_permission(&Role::Guest, Permission::Write));
    }

    #[test]
    fn test_has_all_permissions() {
        let perms = vec![Permission::Read, Permission::Write];
        assert!(has_all_permissions(&Role::User, &perms));
        assert!(!has_all_permissions(&Role::Guest, &perms));
    }

    #[test]
    fn test_has_any_permission() {
        let perms = vec![Permission::Write, Permission::Delete];
        assert!(has_any_permission(&Role::User, &perms)); // Has Write
        assert!(!has_any_permission(&Role::Guest, &perms)); // Has neither
    }

    #[test]
    fn test_check_permission_ok() {
        assert!(check_permission(&Role::User, Permission::Write).is_ok());
    }

    #[test]
    fn test_check_permission_unauthorized() {
        let result = check_permission(&Role::Guest, Permission::Write);
        assert!(result.is_err());
        assert!(matches!(result, Err(AuthError::Unauthorized)));
    }

    #[test]
    fn test_filter_allowed_permissions() {
        let all_perms = vec![
            Permission::Read,
            Permission::Write,
            Permission::Delete,
            Permission::Admin,
        ];

        let user_allowed = filter_allowed_permissions(&Role::User, &all_perms);
        assert_eq!(user_allowed.len(), 2); // Read, Write
        assert!(user_allowed.contains(&Permission::Read));
        assert!(user_allowed.contains(&Permission::Write));
        assert!(!user_allowed.contains(&Permission::Delete));
    }

    #[test]
    fn test_missing_permissions() {
        let required = vec![Permission::Read, Permission::Write, Permission::Delete];
        let missing = missing_permissions(&Role::User, &required);

        assert_eq!(missing.len(), 1);
        assert!(missing.contains(&Permission::Delete));
    }

    #[test]
    fn test_token_permission() {
        let token = Token::new("user123".to_string(), Role::User, 3600);

        assert!(check_token_permission(&token, Permission::Write).is_ok());
        assert!(check_token_permission(&token, Permission::Delete).is_err());
    }
}
