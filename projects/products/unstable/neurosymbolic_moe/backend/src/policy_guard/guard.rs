use crate::moe_core::{ExpertOutput, MoeError};

use super::policy::{Policy, PolicyResult, PolicyType};

const UNSAFE_MARKERS: &[&str] = &["<UNSAFE>", "[BLOCKED]", "HARMFUL_CONTENT"];

#[derive(Debug, Clone)]
pub struct PolicyGuard {
    policies: Vec<Policy>,
}

impl PolicyGuard {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.push(policy);
    }

    pub fn remove_policy(&mut self, id: &str) -> bool {
        let initial_len = self.policies.len();
        self.policies.retain(|p| p.id != id);
        self.policies.len() < initial_len
    }

    pub fn validate(&self, output: &ExpertOutput) -> Vec<PolicyResult> {
        self.policies
            .iter()
            .filter(|p| p.active)
            .map(|p| evaluate_policy(p, output))
            .collect()
    }

    pub fn validate_strict(&self, output: &ExpertOutput) -> Result<(), MoeError> {
        let results = self.validate(output);
        for result in &results {
            if !result.passed {
                let reason = result
                    .reason
                    .clone()
                    .unwrap_or_else(|| "policy check failed".to_string());
                return Err(MoeError::PolicyRejected(format!(
                    "policy '{}' failed: {}",
                    result.policy_id, reason
                )));
            }
        }
        Ok(())
    }

    pub fn active_policy_count(&self) -> usize {
        self.policies.iter().filter(|p| p.active).count()
    }
}

impl Default for PolicyGuard {
    fn default() -> Self {
        Self::new()
    }
}

fn evaluate_policy(policy: &Policy, output: &ExpertOutput) -> PolicyResult {
    match &policy.policy_type {
        PolicyType::ContentFilter => {
            if output.content.is_empty() {
                PolicyResult {
                    policy_id: policy.id.clone(),
                    passed: false,
                    reason: Some("content is empty".to_string()),
                }
            } else {
                PolicyResult {
                    policy_id: policy.id.clone(),
                    passed: true,
                    reason: None,
                }
            }
        }
        PolicyType::SafetyCheck => {
            for marker in UNSAFE_MARKERS {
                if output.content.contains(marker) {
                    return PolicyResult {
                        policy_id: policy.id.clone(),
                        passed: false,
                        reason: Some(format!("content contains unsafe marker: {marker}")),
                    };
                }
            }
            PolicyResult {
                policy_id: policy.id.clone(),
                passed: true,
                reason: None,
            }
        }
        // FormatValidation is intentionally separate from ContentFilter to allow
        // distinct policy identities even though the current check is the same.
        PolicyType::FormatValidation => {
            if output.content.is_empty() {
                PolicyResult {
                    policy_id: policy.id.clone(),
                    passed: false,
                    reason: Some("content is empty".to_string()),
                }
            } else {
                PolicyResult {
                    policy_id: policy.id.clone(),
                    passed: true,
                    reason: None,
                }
            }
        }
        PolicyType::LengthLimit(max) => {
            if output.content.len() > *max {
                PolicyResult {
                    policy_id: policy.id.clone(),
                    passed: false,
                    reason: Some(format!(
                        "content length {} exceeds limit {}",
                        output.content.len(),
                        max
                    )),
                }
            } else {
                PolicyResult {
                    policy_id: policy.id.clone(),
                    passed: true,
                    reason: None,
                }
            }
        }
        PolicyType::Custom(_) => PolicyResult {
            policy_id: policy.id.clone(),
            passed: true,
            reason: None,
        },
    }
}
