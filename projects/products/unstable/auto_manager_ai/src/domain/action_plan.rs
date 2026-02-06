use serde::{Deserialize, Serialize};
use super::types::{ActionStatus, ActionTarget, DryRun, Evidence, RiskLevel};

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

/// The complete action plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionPlan {
    pub version: String,
    pub generated_at: String,
    pub actions: Vec<Action>,
    pub summary: String,
}

impl ActionPlan {
    /// Create a new empty action plan
    pub fn new(summary: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            version: "0.1.0".to_string(),
            generated_at: timestamp.to_string(),
            actions: Vec::new(),
            summary,
        }
    }

    /// Add an action to the plan
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}
