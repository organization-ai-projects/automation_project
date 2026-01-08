// security/src/role.rs
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::Permission;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Moderator,
    User,
    Guest,
}



impl Role {
    /// Retourne les permissions associées à ce rôle
    pub fn permissions(&self) -> &'static [Permission] {
        match self {
            Role::Admin => &[
                Permission::Read,
                Permission::Write,
                Permission::Execute,
                Permission::Delete,
                Permission::Admin,
                Permission::Train,
                Permission::ViewLogs,
                Permission::ConfigureSystem,
            ],
            Role::Moderator => &[
                Permission::Read,
                Permission::Write,
                Permission::Execute,
                Permission::Train,
                Permission::ViewLogs,
            ],
            Role::User => &[Permission::Read, Permission::Write, Permission::Execute],
            Role::Guest => &[Permission::Read],
        }
    }

    /// Vérifie si ce rôle a une permission spécifique
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }

    /// Retourne le niveau de privilège (pour comparaisons)
    pub fn privilege_level(&self) -> u8 {
        match self {
            Role::Admin => 4,
            Role::Moderator => 3,
            Role::User => 2,
            Role::Guest => 1,
        }
    }

    /// Vérifie si ce rôle a plus de privilèges qu'un autre
    pub fn has_higher_privilege_than(&self, other: &Role) -> bool {
        self.privilege_level() > other.privilege_level()
    }

    /// Convertit en string
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Moderator => "moderator",
            Role::User => "user",
            Role::Guest => "guest",
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "moderator" | "mod" => Ok(Role::Moderator),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_has_all_permissions() {
        let admin = Role::Admin;
        for &permission in Permission::all() {
            assert!(
                admin.has_permission(permission),
                "Admin should have {:?} permission",
                permission
            );
        }
    }

    #[test]
    fn test_guest_only_read() {
        let guest = Role::Guest;
        assert!(guest.has_permission(Permission::Read));
        assert!(!guest.has_permission(Permission::Write));
        assert!(!guest.has_permission(Permission::Execute));
        assert!(!guest.has_permission(Permission::Delete));
        assert!(!guest.has_permission(Permission::Admin));
    }

    #[test]
    fn test_moderator_cannot_delete_or_admin() {
        let moderator = Role::Moderator;
        assert!(moderator.has_permission(Permission::Read));
        assert!(moderator.has_permission(Permission::Write));
        assert!(moderator.has_permission(Permission::Execute));
        assert!(!moderator.has_permission(Permission::Delete));
        assert!(!moderator.has_permission(Permission::Admin));
        assert!(!moderator.has_permission(Permission::ConfigureSystem));
    }

    #[test]
    fn test_user_basic_permissions() {
        let user = Role::User;
        assert!(user.has_permission(Permission::Read));
        assert!(user.has_permission(Permission::Write));
        assert!(user.has_permission(Permission::Execute));
        assert!(!user.has_permission(Permission::Delete));
        assert!(!user.has_permission(Permission::Train));
    }

    #[test]
    fn test_privilege_levels() {
        assert!(Role::Admin.privilege_level() > Role::Moderator.privilege_level());
        assert!(Role::Moderator.privilege_level() > Role::User.privilege_level());
        assert!(Role::User.privilege_level() > Role::Guest.privilege_level());
    }

    #[test]
    fn test_higher_privilege() {
        assert!(Role::Admin.has_higher_privilege_than(&Role::User));
        assert!(Role::Moderator.has_higher_privilege_than(&Role::Guest));
        assert!(!Role::Guest.has_higher_privilege_than(&Role::User));
        assert!(!Role::User.has_higher_privilege_than(&Role::Admin));
    }

    #[test]
    fn test_role_from_str() {
        assert_eq!(Role::from_str("admin"), Ok(Role::Admin));
        assert_eq!(Role::from_str("ADMIN"), Ok(Role::Admin));
        assert_eq!(Role::from_str("moderator"), Ok(Role::Moderator));
        assert_eq!(Role::from_str("mod"), Ok(Role::Moderator));
        assert_eq!(Role::from_str("user"), Ok(Role::User));
        assert_eq!(Role::from_str("guest"), Ok(Role::Guest));
        assert!(Role::from_str("invalid").is_err());
    }

    #[test]
    fn test_permission_from_str() {
        assert_eq!(Permission::from_str("read"), Ok(Permission::Read));
        assert_eq!(Permission::from_str("READ"), Ok(Permission::Read));
        assert_eq!(Permission::from_str("write"), Ok(Permission::Write));
        assert_eq!(Permission::from_str("view_logs"), Ok(Permission::ViewLogs));
        assert_eq!(Permission::from_str("viewlogs"), Ok(Permission::ViewLogs));
        assert!(Permission::from_str("invalid").is_err());
    }
}
