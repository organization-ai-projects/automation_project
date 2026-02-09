//! projects/products/varina/backend/src/autopilot/autopilot_report.rs
use serde::{Deserialize, Serialize};

use crate::{
    autopilot::{AutopilotMode, AutopilotPlan},
    classified_changes::ClassifiedChanges,
};

/// Structure representing an execution report of the autopilot.
/// Combines the plan, classified changes, and logs for complete tracking.
/// Execution report (plan + actions performed or refused).
#[derive(Debug, Clone, Serialize, Deserialize)] // Added Serialize to allow JSON conversion
pub struct AutopilotReport {
    pub mode: AutopilotMode,
    pub branch: String,
    pub detached_head: bool,
    pub changes: Vec<String>,
    pub classified: ClassifiedChanges,
    pub plan: AutopilotPlan,
    pub applied: bool,
    pub logs: Vec<String>,
}

impl AutopilotReport {
    /// Add a log entry to the report.
    pub fn add_log(&mut self, entry: String) {
        self.logs.push(entry);
    }
}
