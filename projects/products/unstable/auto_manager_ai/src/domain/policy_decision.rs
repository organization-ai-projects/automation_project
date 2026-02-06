// projects/products/unstable/auto_manager_ai/src/domain/policy_decision.rs

use serde::{Deserialize, Serialize};
use super::policy_decision_type::PolicyDecisionType;

/// A policy decision with reasoning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub action_id: String,
    pub decision: PolicyDecisionType,
    pub reason: String,
}
