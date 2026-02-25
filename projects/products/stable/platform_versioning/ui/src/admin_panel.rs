// projects/products/stable/platform_versioning/ui/src/admin_panel.rs
use serde::{Deserialize, Serialize};

use crate::issue_panel::IssuePanel;
use crate::permission_panel::PermissionPanel;
use crate::role_view::RoleView;
use crate::slice_panel::SlicePanel;

/// Top-level admin governance panel for platform-versioning UI.
///
/// Aggregates the permission, issue, and slice management panels into a
/// single view. Visibility is controlled by the user's [`RoleView`]:
///
/// - **Admin**: all three sub-panels are visible and writable.
/// - **Developer**: only the issue panel is shown, in read-only mode,
///   and only issues assigned to or shared with the developer are listed.
///
/// This panel is not an IDE: it contains no file editor, no terminal, and
/// no Monaco editor component.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AdminPanel {
    /// The current user's role.
    pub role: RoleView,
    /// Permission management sub-panel (admin only).
    pub permission_panel: PermissionPanel,
    /// Issue assignment and sharing sub-panel.
    pub issue_panel: IssuePanel,
    /// Slice definition management sub-panel (admin only).
    pub slice_panel: SlicePanel,
}

impl AdminPanel {
    /// Creates an admin panel initialised for the given role.
    pub fn for_role(role: RoleView) -> Self {
        Self {
            role,
            permission_panel: PermissionPanel::default(),
            issue_panel: IssuePanel::default(),
            slice_panel: SlicePanel::default(),
        }
    }

    /// Returns `true` if the permission panel should be visible to this user.
    pub fn show_permission_panel(&self) -> bool {
        self.role.is_admin()
    }

    /// Returns `true` if the slice panel should be visible to this user.
    pub fn show_slice_panel(&self) -> bool {
        self.role.is_admin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_sees_all_panels() {
        let panel = AdminPanel::for_role(RoleView::Admin);
        assert!(panel.show_permission_panel());
        assert!(panel.show_slice_panel());
    }

    #[test]
    fn developer_sees_only_issue_panel() {
        let panel = AdminPanel::for_role(RoleView::Developer);
        assert!(!panel.show_permission_panel());
        assert!(!panel.show_slice_panel());
    }

    #[test]
    fn default_role_is_developer() {
        let panel = AdminPanel::default();
        assert_eq!(panel.role, RoleView::Developer);
    }
}
