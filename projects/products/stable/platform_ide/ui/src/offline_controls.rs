// projects/products/stable/platform_ide/ui/src/offline_controls.rs
use serde::{Deserialize, Serialize};

/// The offline controls view state.
///
/// These controls are shown **only** when the platform signals that offline
/// mode is admin-approved. They are hidden by default.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OfflineControls {
    /// Whether the offline controls are visible.
    pub visible: bool,
    /// Optional notice text from the platform.
    pub notice: Option<String>,
}

impl OfflineControls {
    /// Shows the offline controls with an optional notice.
    pub fn show(&mut self, notice: Option<String>) {
        self.visible = true;
        self.notice = notice;
    }

    /// Hides the offline controls.
    pub fn hide(&mut self) {
        self.visible = false;
        self.notice = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_by_default() {
        let controls = OfflineControls::default();
        assert!(!controls.visible);
    }

    #[test]
    fn show_makes_visible() {
        let mut controls = OfflineControls::default();
        controls.show(Some("Admin approved.".to_string()));
        assert!(controls.visible);
        assert_eq!(controls.notice.as_deref(), Some("Admin approved."));
    }

    #[test]
    fn hide_clears_state() {
        let mut controls = OfflineControls::default();
        controls.show(Some("notice".to_string()));
        controls.hide();
        assert!(!controls.visible);
        assert!(controls.notice.is_none());
    }
}
