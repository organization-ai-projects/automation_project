// projects/products/stable/platform_versioning/ui/src/role_view.rs
use serde::{Deserialize, Serialize};

/// The role a user has in the platform, controlling what they can see.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RoleView {
    /// The user has admin privileges and may see and manage everything.
    Admin,
    /// The user is a developer and may only see what they are assigned to or
    /// what has been explicitly shared with them.
    #[default]
    Developer,
}

impl RoleView {
    /// Returns `true` if this role has admin-level access.
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }

    /// Returns the human-readable label for this role.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Admin => "Admin",
            Self::Developer => "Developer",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_is_admin() {
        assert!(RoleView::Admin.is_admin());
    }

    #[test]
    fn developer_is_not_admin() {
        assert!(!RoleView::Developer.is_admin());
    }

    #[test]
    fn label_is_human_readable() {
        assert_eq!(RoleView::Admin.label(), "Admin");
        assert_eq!(RoleView::Developer.label(), "Developer");
    }

    #[test]
    fn default_is_developer() {
        assert_eq!(RoleView::default(), RoleView::Developer);
    }
}
