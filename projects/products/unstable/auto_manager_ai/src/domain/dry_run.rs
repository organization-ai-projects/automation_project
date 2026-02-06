// projects/products/unstable/auto_manager_ai/src/domain/dry_run.rs

use super::dry_run_step::DryRunStep;
use serde::{Deserialize, Serialize};

/// Dry-run information for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRun {
    pub steps: Vec<DryRunStep>,
}
