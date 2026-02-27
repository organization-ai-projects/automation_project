use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptivePolicyAction {
    IncreaseExecutionBudget,
    IncreaseRemediationCycles,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptivePolicyDecision {
    pub action: AdaptivePolicyAction,
    pub reason_code: String,
    pub trigger_signature: String,
    pub previous_value: u32,
    pub new_value: u32,
}
