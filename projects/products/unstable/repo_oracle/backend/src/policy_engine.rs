use crate::diagnostics::Error;
use crate::policy::{Policy, PolicyRuleKind};
use crate::policy_result::{PolicyResult, PolicyViolation};
use crate::snapshot::Snapshot;

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(snapshot: &Snapshot, policy: &Policy) -> Result<PolicyResult, Error> {
        let mut violations = Vec::new();

        for rule in &policy.rules {
            match &rule.kind {
                PolicyRuleKind::ForbidDependency { from, to } => {
                    if let Some(deps) = snapshot.crate_graph.direct_deps(from) {
                        if deps.iter().any(|d| d == to) {
                            violations.push(PolicyViolation {
                                rule_description: rule.description.clone(),
                                detail: format!("{from} depends on {to}, which is forbidden"),
                            });
                        }
                    }
                }
                PolicyRuleKind::RequireDependency { from, to } => {
                    let has_dep = snapshot
                        .crate_graph
                        .direct_deps(from)
                        .map(|deps| deps.iter().any(|d| d == to))
                        .unwrap_or(false);

                    if !has_dep {
                        violations.push(PolicyViolation {
                            rule_description: rule.description.clone(),
                            detail: format!("{from} does not depend on {to}, but it is required"),
                        });
                    }
                }
                PolicyRuleKind::MaxDependencies { crate_name, max } => {
                    if let Some(deps) = snapshot.crate_graph.direct_deps(crate_name) {
                        if deps.len() > *max {
                            violations.push(PolicyViolation {
                                rule_description: rule.description.clone(),
                                detail: format!(
                                    "{crate_name} has {} dependencies, exceeding max of {max}",
                                    deps.len()
                                ),
                            });
                        }
                    }
                }
            }
        }

        let passed = violations.is_empty();
        Ok(PolicyResult {
            policy_name: policy.name.clone(),
            violations,
            passed,
        })
    }
}
