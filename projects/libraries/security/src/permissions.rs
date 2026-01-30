// projects/libraries/security/src/permissions.rs
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::PermissionError;
use crate::role::Role;
use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read code, view projects
    Read,

    /// Write/modify code
    Write,

    /// Execute code generation, analysis, etc.
    Execute,

    /// Delete projects/files
    Delete,

    /// Administer (manage users, permissions, settings)
    Admin,

    /// Train/adjust models
    Train,

    /// Access logs and metrics
    ViewLogs,

    /// Modify system configuration
    ConfigureSystem,
}

impl Permission {
    /// Returns all available permissions
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

    /// Converts to string
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

// Display implementation for Permission
impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Checks if a role has a specific permission
pub fn has_permission(role: &Role, required_permission: Permission) -> bool {
    role.has_permission(required_permission)
}

/// Checks if a role has all specified permissions
pub fn has_all_permissions(role: &Role, required_permissions: &[Permission]) -> bool {
    required_permissions
        .iter()
        .all(|&perm| role.has_permission(perm))
}

/// Checks if a role has at least one of the specified permissions
pub fn has_any_permission(role: &Role, required_permissions: &[Permission]) -> bool {
    required_permissions
        .iter()
        .any(|&perm| role.has_permission(perm))
}

/// Checks if a role has a specific permission, returns an error otherwise
pub fn check_permission(
    role: &Role,
    required_permission: Permission,
) -> Result<(), PermissionError> {
    if has_permission(role, required_permission) {
        Ok(())
    } else {
        Err(PermissionError::Unauthorized)
    }
}

/// Checks if a role has all required permissions
pub fn check_all_permissions(
    role: &Role,
    required_permissions: &[Permission],
) -> Result<(), PermissionError> {
    if has_all_permissions(role, required_permissions) {
        Ok(())
    } else {
        Err(PermissionError::Unauthorized)
    }
}

/// Checks if a valid token has a specific permission
pub fn check_token_permission(
    token: &Token,
    required_permission: Permission,
) -> Result<(), PermissionError> {
    token
        .validate_token()
        .map_err(|_| PermissionError::Unauthorized)?;
    check_permission(&token.role, required_permission)
}

/// Checks if a valid token has all required permissions
pub fn check_token_all_permissions(
    token: &Token,
    required_permissions: &[Permission],
) -> Result<(), PermissionError> {
    token
        .validate_token()
        .map_err(|_| PermissionError::Unauthorized)?;
    check_all_permissions(&token.role, required_permissions)
}

/// Filters a list of permissions, keeping only those a role possesses
pub fn filter_allowed_permissions(role: &Role, permissions: &[Permission]) -> Vec<Permission> {
    permissions
        .iter()
        .copied()
        .filter(|&perm| role.has_permission(perm))
        .collect()
}

/// Returns the missing permissions for a role
pub fn missing_permissions(role: &Role, required_permissions: &[Permission]) -> Vec<Permission> {
    required_permissions
        .iter()
        .copied()
        .filter(|&perm| !role.has_permission(perm))
        .collect()
}
