use crate::orchestrator::ContinuousGovernancePolicy;

#[test]
fn constructor_sets_thresholds_and_defaults() {
    let policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.4, 0.1, true);
    assert!((policy.min_expert_success_rate - 0.8).abs() < f64::EPSILON);
    assert!((policy.min_routing_accuracy - 0.9).abs() < f64::EPSILON);
    assert!((policy.low_score_threshold - 0.4).abs() < f64::EPSILON);
    assert!((policy.regression_drop_threshold - 0.1).abs() < f64::EPSILON);
    assert!(policy.block_on_human_review);
    assert!(!policy.auto_promote_on_pass);
}

#[test]
fn auto_promote_builder_updates_flag() {
    let policy =
        ContinuousGovernancePolicy::new(0.8, 0.9, 0.4, 0.1, false).with_auto_promote_on_pass(true);
    assert!(policy.auto_promote_on_pass);
}
