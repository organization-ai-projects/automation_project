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

    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }

    pub fn privilege_level(&self) -> u8 {
        match self {
            Role::Admin => 4,
            Role::Moderator => 3,
            Role::User => 2,
            Role::Guest => 1,
        }
    }

    pub fn has_higher_privilege_than(&self, other: &Role) -> bool {
        self.privilege_level() > other.privilege_level()
    }

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

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
