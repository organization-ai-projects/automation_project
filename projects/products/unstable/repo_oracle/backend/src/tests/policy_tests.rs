use crate::policy::{Policy, PolicyRule, PolicyRuleKind};
use crate::policy_engine::PolicyEngine;
use crate::scanner::WorkspaceScanner;

fn fixture_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/src/tests/fixtures/sample_workspace")
}

fn scan_fixture() -> crate::snapshot::Snapshot {
    WorkspaceScanner::scan(&fixture_path()).unwrap()
}

#[test]
fn forbid_dependency_passing() {
    let snapshot = scan_fixture();
    let policy = Policy {
        name: "no_cycle".to_string(),
        rules: vec![PolicyRule {
            kind: PolicyRuleKind::ForbidDependency {
                from: "crate_a".to_string(),
                to: "crate_c".to_string(),
            },
            description: "crate_a must not depend on crate_c".to_string(),
        }],
    };

    let result = PolicyEngine::evaluate(&snapshot, &policy).unwrap();
    assert!(result.passed);
    assert!(result.violations.is_empty());
}

#[test]
fn forbid_dependency_violation() {
    let snapshot = scan_fixture();
    let policy = Policy {
        name: "forbid_test".to_string(),
        rules: vec![PolicyRule {
            kind: PolicyRuleKind::ForbidDependency {
                from: "crate_b".to_string(),
                to: "crate_a".to_string(),
            },
            description: "crate_b must not depend on crate_a".to_string(),
        }],
    };

    let result = PolicyEngine::evaluate(&snapshot, &policy).unwrap();
    assert!(!result.passed);
    assert_eq!(result.violations.len(), 1);
    assert!(result.violations[0].detail.contains("forbidden"));
}

#[test]
fn max_dependencies_passing() {
    let snapshot = scan_fixture();
    let policy = Policy {
        name: "max_deps".to_string(),
        rules: vec![PolicyRule {
            kind: PolicyRuleKind::MaxDependencies {
                crate_name: "crate_c".to_string(),
                max: 5,
            },
            description: "crate_c max 5 deps".to_string(),
        }],
    };

    let result = PolicyEngine::evaluate(&snapshot, &policy).unwrap();
    assert!(result.passed);
}

#[test]
fn max_dependencies_violation() {
    let snapshot = scan_fixture();
    let policy = Policy {
        name: "strict_deps".to_string(),
        rules: vec![PolicyRule {
            kind: PolicyRuleKind::MaxDependencies {
                crate_name: "crate_c".to_string(),
                max: 1,
            },
            description: "crate_c max 1 dep".to_string(),
        }],
    };

    let result = PolicyEngine::evaluate(&snapshot, &policy).unwrap();
    assert!(!result.passed);
    assert_eq!(result.violations.len(), 1);
    assert!(result.violations[0].detail.contains("exceeding"));
}

#[test]
fn require_dependency_passing() {
    let snapshot = scan_fixture();
    let policy = Policy {
        name: "require_test".to_string(),
        rules: vec![PolicyRule {
            kind: PolicyRuleKind::RequireDependency {
                from: "crate_b".to_string(),
                to: "crate_a".to_string(),
            },
            description: "crate_b must depend on crate_a".to_string(),
        }],
    };

    let result = PolicyEngine::evaluate(&snapshot, &policy).unwrap();
    assert!(result.passed);
}

#[test]
fn require_dependency_violation() {
    let snapshot = scan_fixture();
    let policy = Policy {
        name: "require_test".to_string(),
        rules: vec![PolicyRule {
            kind: PolicyRuleKind::RequireDependency {
                from: "crate_a".to_string(),
                to: "crate_b".to_string(),
            },
            description: "crate_a must depend on crate_b".to_string(),
        }],
    };

    let result = PolicyEngine::evaluate(&snapshot, &policy).unwrap();
    assert!(!result.passed);
    assert_eq!(result.violations.len(), 1);
    assert!(result.violations[0].detail.contains("required"));
}
