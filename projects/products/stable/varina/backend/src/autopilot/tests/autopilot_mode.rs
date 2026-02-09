//! projects/products/stable/varina/backend/src/autopilot/tests/mod.rs

use crate::autopilot::AutopilotMode;

#[test]
fn test_autopilot_mode_usage() {
    let mode = AutopilotMode::DryRun;
    assert_eq!(mode, AutopilotMode::DryRun);

    let mode = AutopilotMode::ApplySafe;
    assert_eq!(mode, AutopilotMode::ApplySafe);
}
