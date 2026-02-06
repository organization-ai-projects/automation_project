// projects/products/unstable/auto_manager_ai/src/domain/action.rs

use serde::{Deserialize, Serialize};
use super::action_status::ActionStatus;
use super::action_target::ActionTarget;
use super::dry_run::DryRun;
use super::evidence::Evidence;
use super::risk_level::RiskLevel;

/// An action in the action plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub action_type: String,
    pub status: ActionStatus,
    pub target: ActionTarget,
    pub justification: String,
    pub risk_level: RiskLevel,
    pub required_checks: Vec<String>,
    pub confidence: f64,
    pub evidence: Vec<Evidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_inputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<DryRun>,
}
