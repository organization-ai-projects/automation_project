// projects/products/unstable/auto_manager_ai/src/domain/dry_run_step.rs

use serde::{Deserialize, Serialize};

/// Dry-run step for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunStep {
    pub tool: String,
    pub command: String,
    pub expected: String,
    pub failure_modes: Vec<String>,
}
