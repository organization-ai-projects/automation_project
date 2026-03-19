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
