use crate::domain::{AdaptivePolicyAction, AdaptivePolicyDecision};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdaptivePolicyConfig {
    pub max_execution_iterations_cap: u32,
    pub max_remediation_cycles_cap: u32,
}

impl Default for AdaptivePolicyConfig {
    fn default() -> Self {
        Self {
            max_execution_iterations_cap: 5,
            max_remediation_cycles_cap: 3,
        }
    }
}

pub fn maybe_increase_execution_budget(
    current: u32,
    trigger_signature: &str,
    cfg: AdaptivePolicyConfig,
) -> Option<AdaptivePolicyDecision> {
    if current >= cfg.max_execution_iterations_cap {
        return None;
    }
    Some(AdaptivePolicyDecision {
        action: AdaptivePolicyAction::IncreaseExecutionBudget,
        reason_code: "ADAPTIVE_RETRY_BUDGET_INCREASED".to_string(),
        trigger_signature: trigger_signature.to_string(),
        previous_value: current,
        new_value: current.saturating_add(1),
    })
}

pub fn maybe_increase_remediation_cycles(
    current: u32,
    trigger_signature: &str,
    cfg: AdaptivePolicyConfig,
) -> Option<AdaptivePolicyDecision> {
    if current >= cfg.max_remediation_cycles_cap {
        return None;
    }
    Some(AdaptivePolicyDecision {
        action: AdaptivePolicyAction::IncreaseRemediationCycles,
        reason_code: "ADAPTIVE_REMEDIATION_CYCLES_INCREASED".to_string(),
        trigger_signature: trigger_signature.to_string(),
        previous_value: current,
        new_value: current.saturating_add(1),
    })
}
