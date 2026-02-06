// projects/products/unstable/auto_manager_ai/src/domain/action_plan.rs

use super::action::Action;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use crate::domain::action_plan::ActionPlan;
    use common_json::{from_str, to_string_pretty};

    #[test]
    fn test_action_plan_new() {
        let plan = ActionPlan::new("Test plan".to_string());
        assert_eq!(plan.version, "0.1.0");
        assert_eq!(plan.summary, "Test plan");
        assert_eq!(plan.actions.len(), 0);
    }

    #[test]
    fn test_action_plan_serialization() {
        let plan = ActionPlan::new("Test plan".to_string());

        let json = to_string_pretty(&plan).expect("Failed to serialize");
        assert!(json.contains("version"));
        assert!(json.contains("generated_at"));
        assert!(json.contains("actions"));
        assert!(json.contains("summary"));

        let _deserialized: ActionPlan = from_str(&json).expect("Failed to deserialize");
    }

    #[test]
    fn test_action_plan_round_trip() {
        let plan = ActionPlan::new("Round trip test".to_string());
        let json = to_string_pretty(&plan).expect("Failed to serialize");
        let deserialized: ActionPlan = from_str(&json).expect("Failed to deserialize");

        assert_eq!(plan.version, deserialized.version);
        assert_eq!(plan.summary, deserialized.summary);
        assert_eq!(plan.actions.len(), deserialized.actions.len());
    }
}
