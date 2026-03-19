// projects/products/stable/platform_ide/ui/src/verification_view.rs
use crate::finding_entry::FindingEntry;
use serde::{Deserialize, Serialize};

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
