// projects/products/unstable/platform_ide/ui/src/verification_view.rs
use serde::{Deserialize, Serialize};

/// A verification finding entry for display.
///
/// Only safe summaries are shown; forbidden paths are never presented.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingEntry {
    /// "error", "warning", or "info".
    pub severity: String,
    /// Safe human-readable summary.
    pub summary: String,
    /// Optional allowed file path (absent for contract-level/global findings).
    pub path: Option<String>,
    /// Optional line number (absent when path is not present).
    pub line: Option<u32>,
}

/// The verification results view state.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VerificationView {
    /// Whether the verification run passed.
    pub healthy: bool,
    /// The filtered findings to display.
    pub findings: Vec<FindingEntry>,
    /// Whether a verification run is in progress.
    pub running: bool,
}

impl VerificationView {
    /// Loads verification results.
    pub fn load(&mut self, healthy: bool, findings: Vec<FindingEntry>) {
        self.healthy = healthy;
        self.findings = findings;
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sets_results() {
        let mut view = VerificationView::default();
        view.load(
            false,
            vec![FindingEntry {
                severity: "error".to_string(),
                summary: "undefined variable".to_string(),
                path: Some("src/main.rs".to_string()),
                line: Some(10),
            }],
        );
        assert!(!view.healthy);
        assert_eq!(view.findings.len(), 1);
        assert!(!view.running);
    }

    #[test]
    fn healthy_result_is_empty() {
        let mut view = VerificationView::default();
        view.load(true, vec![]);
        assert!(view.healthy);
        assert!(view.findings.is_empty());
    }
}
