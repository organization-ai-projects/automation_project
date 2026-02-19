use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Delete,
    Admin,
    Train,
    ViewLogs,
    ConfigureSystem,
}

impl Permission {
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

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
