//! projects/products/varina/backend/src/autopilot/autopilot_mode.rs
use serde::{Deserialize, Serialize};

/// Autopilot execution mode.
/// - `DryRun`: Generates a plan without applying changes.
/// - `ApplySafe`: Applies changes only if checks pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutopilotMode {
    DryRun,
    ApplySafe,
}

#[cfg(test)]
mod tests {
    use crate::autopilot::autopilot_mode::AutopilotMode;

    #[test]
    fn test_autopilot_mode_usage() {
        let mode = AutopilotMode::DryRun;
        assert_eq!(mode, AutopilotMode::DryRun);

        let mode = AutopilotMode::ApplySafe;
        assert_eq!(mode, AutopilotMode::ApplySafe);
    }
}
