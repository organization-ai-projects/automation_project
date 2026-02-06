// projects/products/unstable/auto_manager_ai/src/domain/policy_decision_type.rs

use serde::{Deserialize, Serialize};

/// Policy decision for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyDecisionType {
    Allow,
    Deny,
    #[serde(rename = "needs_input")]
    NeedsInput,
}
