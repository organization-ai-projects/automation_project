use crate::adaptive_policy::{
    AdaptivePolicyConfig, maybe_increase_execution_budget, maybe_increase_remediation_cycles,
};
use crate::domain::AdaptivePolicyAction;

#[test]
fn execution_budget_increase_is_bounded_and_machine_readable() {
    let decision =
        maybe_increase_execution_budget(1, "stage=execution", AdaptivePolicyConfig::default())
            .expect("decision should be produced");
    assert_eq!(
        decision.action,
        AdaptivePolicyAction::IncreaseExecutionBudget
    );
    assert_eq!(decision.reason_code, "ADAPTIVE_RETRY_BUDGET_INCREASED");
    assert_eq!(decision.previous_value, 1);
    assert_eq!(decision.new_value, 2);
}

#[test]
fn remediation_budget_increase_is_bounded_and_machine_readable() {
    let decision =
        maybe_increase_remediation_cycles(0, "stage=validation", AdaptivePolicyConfig::default())
            .expect("decision should be produced");
    assert_eq!(
        decision.action,
        AdaptivePolicyAction::IncreaseRemediationCycles
    );
    assert_eq!(
        decision.reason_code,
        "ADAPTIVE_REMEDIATION_CYCLES_INCREASED"
    );
    assert_eq!(decision.previous_value, 0);
    assert_eq!(decision.new_value, 1);
}

#[test]
fn no_decision_when_cap_is_reached() {
    let cfg = AdaptivePolicyConfig {
        max_execution_iterations_cap: 2,
        max_remediation_cycles_cap: 1,
    };
    assert!(maybe_increase_execution_budget(2, "x", cfg).is_none());
    assert!(maybe_increase_remediation_cycles(1, "x", cfg).is_none());
}
