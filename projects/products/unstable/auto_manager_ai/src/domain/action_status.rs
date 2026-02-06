// projects/products/unstable/auto_manager_ai/src/domain/action_status.rs

use serde::{Deserialize, Serialize};

/// Status of an action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionStatus {
    Proposed,
    #[serde(rename = "needs_input")]
    NeedsInput,
    #[serde(rename = "blocked_by_policy")]
    BlockedByPolicy,
}
