// projects/products/unstable/auto_manager_ai/src/domain/action_plan.rs

use serde::{Deserialize, Serialize};
use super::action::Action;

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
