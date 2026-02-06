use serde::{Deserialize, Serialize};
use super::policy::PolicyDecision;

/// Status of the run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Success,
    Failure,
}

/// Output information from the run
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunOutput {
    pub actions_proposed: usize,
    pub actions_allowed: usize,
    pub actions_denied: usize,
    pub actions_needs_input: usize,
}

/// Complete run report
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunReport {
    pub product: String,
    pub version: String,
    pub run_id: String,
    pub timestamp: String,
    pub status: RunStatus,
    pub output: RunOutput,
    pub policy_decisions: Vec<PolicyDecision>,
    pub errors: Vec<String>,
}

impl RunReport {
    /// Create a new run report
    pub fn new(run_id: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            product: "auto_manager_ai".to_string(),
            version: "0.1.0".to_string(),
            run_id,
            timestamp: timestamp.to_string(),
            status: RunStatus::Success,
            output: RunOutput {
                actions_proposed: 0,
                actions_allowed: 0,
                actions_denied: 0,
                actions_needs_input: 0,
            },
            policy_decisions: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Add a policy decision to the report
    pub fn add_decision(&mut self, decision: PolicyDecision) {
        use super::policy::PolicyDecisionType;
        match decision.decision {
            PolicyDecisionType::Allow => self.output.actions_allowed += 1,
            PolicyDecisionType::Deny => self.output.actions_denied += 1,
            PolicyDecisionType::NeedsInput => self.output.actions_needs_input += 1,
        }
        self.policy_decisions.push(decision);
    }

    /// Add an error to the report
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.status = RunStatus::Failure;
    }
}
