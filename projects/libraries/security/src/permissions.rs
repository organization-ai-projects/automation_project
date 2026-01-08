use std::str::FromStr;

use serde::{Deserialize, Serialize};

// projects/libraries/security/src/permissions.rs
use crate::AuthError;
use crate::role::Role;
use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Lire le code, voir les projets
    Read,

    /// Écrire/modifier du code
    Write,

    /// Exécuter la génération de code, l'analyse, etc.
    Execute,

    /// Supprimer des projets/fichiers
    Delete,

    /// Administrer (gérer users, permissions, settings)
    Admin,

    /// Entraîner/ajuster les modèles
    Train,

    /// Accéder aux logs et métriques
    ViewLogs,

    /// Modifier la configuration système
    ConfigureSystem,
}

impl Permission {
    /// Retourne toutes les permissions disponibles
    pub fn all() -> &'static [Permission] {
        &[
            Permission::Read,
            Permission::Write,
            Permission::Execute,
            Permission::Delete,
            Permission::Admin,
            Permission::Train,
            Permission::ViewLogs,
            Permission::ConfigureSystem,
        ]
    }

    /// Convertit en string
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::Read => "read",
            Permission::Write => "write",
            Permission::Execute => "execute",
            Permission::Delete => "delete",
            Permission::Admin => "admin",
            Permission::Train => "train",
            Permission::ViewLogs => "view_logs",
            Permission::ConfigureSystem => "configure_system",
        }
    }
}

impl FromStr for Permission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "read" => Ok(Permission::Read),
            "write" => Ok(Permission::Write),
            "execute" => Ok(Permission::Execute),
            "delete" => Ok(Permission::Delete),
            "admin" => Ok(Permission::Admin),
            "train" => Ok(Permission::Train),
            "viewlogs" | "view_logs" => Ok(Permission::ViewLogs),
            "configuresystem" | "configure_system" => Ok(Permission::ConfigureSystem),
            _ => Err(()),
        }
    }
}

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
        // Token::new -> Result + user_id numérique + durée en ms
        let token = Token::new("1".to_string(), Role::User, 3_600_000).unwrap();

        assert!(check_token_permission(&token, Permission::Write).is_ok());
        assert!(check_token_permission(&token, Permission::Delete).is_err());
    }
}
