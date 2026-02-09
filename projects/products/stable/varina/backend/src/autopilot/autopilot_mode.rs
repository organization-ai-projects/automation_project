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
