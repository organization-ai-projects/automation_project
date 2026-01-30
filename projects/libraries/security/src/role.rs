// projects/libraries/security/src/role.rs
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
    /// Returns the permissions associated with this role
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

    /// Checks if this role has a specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }

    /// Returns the privilege level (for comparisons)
    pub fn privilege_level(&self) -> u8 {
        match self {
            Role::Admin => 4,
            Role::Moderator => 3,
            Role::User => 2,
            Role::Guest => 1,
        }
    }

    /// Checks if this role has more privileges than another
    pub fn has_higher_privilege_than(&self, other: &Role) -> bool {
        self.privilege_level() > other.privilege_level()
    }

    /// Converts to string
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

// Implémentation de Display pour Role
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
