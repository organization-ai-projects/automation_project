use crate::moe_core::{ExpertId, ExpertOutput};
use crate::policy_guard::{Policy, PolicyGuard, PolicyType};
use std::collections::HashMap;

fn make_output(content: &str) -> ExpertOutput {
    ExpertOutput {
        expert_id: ExpertId::new("e1"),
        content: content.to_string(),
        confidence: 0.9,
        metadata: HashMap::new(),
        trace: Vec::new(),
    }
}

fn make_policy(id: &str, policy_type: PolicyType) -> Policy {
    Policy {
        id: id.to_string(),
        name: id.to_string(),
        description: "test policy".to_string(),
        policy_type,
        active: true,
    }
}

#[test]
fn validate_with_passing_policies() {
    let mut guard = PolicyGuard::new();
    guard.add_policy(make_policy("p1", PolicyType::ContentFilter));
    guard.add_policy(make_policy("p2", PolicyType::SafetyCheck));
    let output = make_output("safe content");
    let results = guard.validate(&output);
    assert!(results.iter().all(|result| result.passed));
}

#[test]
fn validate_strict_with_failing_policy() {
    let mut guard = PolicyGuard::new();
    guard.add_policy(make_policy("p1", PolicyType::SafetyCheck));
    let output = make_output("this is <UNSAFE> content");
    let result = guard.validate_strict(&output);
    assert!(result.is_err());
}

#[test]
fn length_limit_policy() {
    let mut guard = PolicyGuard::new();
    guard.add_policy(make_policy("p1", PolicyType::LengthLimit(5)));
    let short_output = make_output("hi");
    let short_results = guard.validate(&short_output);
    assert!(short_results[0].passed);

    let long_output = make_output("this is way too long");
    let long_results = guard.validate(&long_output);
    assert!(!long_results[0].passed);
}
