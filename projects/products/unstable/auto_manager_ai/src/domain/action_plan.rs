// projects/products/unstable/auto_manager_ai/src/domain/action_plan.rs

use super::action::Action;
use serde::{Deserialize, Serialize};

/// The complete action plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionPlan {
    pub schema_version: String,
    pub producer: String,
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
            schema_version: "1".to_string(),
            producer: "auto_manager_ai".to_string(),
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
    use common_json::{Json, JsonAccess, from_str, to_string_pretty};

    #[test]
    fn test_action_plan_new() {
        let plan = ActionPlan::new("Test plan".to_string());
        assert_eq!(plan.schema_version, "1");
        assert_eq!(plan.producer, "auto_manager_ai");
        assert_eq!(plan.version, "0.1.0");
        assert_eq!(plan.summary, "Test plan");
        assert_eq!(plan.actions.len(), 0);
    }

    #[test]
    fn test_action_plan_serialization() {
        let plan = ActionPlan::new("Test plan".to_string());

        let json = to_string_pretty(&plan).expect("Failed to serialize");
        let parsed: Json = from_str(&json).expect("Failed to parse JSON");

        // Verify specific fields exist and have correct types
        let schema_version = parsed
            .get_field("schema_version")
            .expect("schema_version field should exist");
        assert!(
            matches!(schema_version, Json::String(_)),
            "schema_version should be a string"
        );
        assert_eq!(
            schema_version.as_str(),
            Some("1"),
            "schema_version should be 1"
        );

        let producer = parsed
            .get_field("producer")
            .expect("producer field should exist");
        assert!(
            matches!(producer, Json::String(_)),
            "producer should be a string"
        );
        assert_eq!(
            producer.as_str(),
            Some("auto_manager_ai"),
            "producer should match"
        );

        let version = parsed
            .get_field("version")
            .expect("version field should exist");
        assert!(
            matches!(version, Json::String(_)),
            "version should be a string"
        );
        assert_eq!(version.as_str(), Some("0.1.0"), "version should be 0.1.0");

        let generated_at = parsed
            .get_field("generated_at")
            .expect("generated_at field should exist");
        assert!(
            matches!(generated_at, Json::String(_)),
            "generated_at should be a string"
        );

        let actions = parsed
            .get_field("actions")
            .expect("actions field should exist");
        assert!(
            matches!(actions, Json::Array(_)),
            "actions should be an array"
        );
        assert_eq!(
            actions.as_array().map(|a| a.len()),
            Some(0),
            "actions should be empty"
        );

        let summary = parsed
            .get_field("summary")
            .expect("summary field should exist");
        assert!(
            matches!(summary, Json::String(_)),
            "summary should be a string"
        );
        assert_eq!(summary.as_str(), Some("Test plan"), "summary should match");

        let _deserialized: ActionPlan = from_str(&json).expect("Failed to deserialize");
    }

    #[test]
    fn test_action_plan_round_trip() {
        let plan = ActionPlan::new("Round trip test".to_string());
        let json = to_string_pretty(&plan).expect("Failed to serialize");
        let deserialized: ActionPlan = from_str(&json).expect("Failed to deserialize");

        assert_eq!(plan.schema_version, deserialized.schema_version);
        assert_eq!(plan.producer, deserialized.producer);
        assert_eq!(plan.version, deserialized.version);
        assert_eq!(plan.summary, deserialized.summary);
        assert_eq!(plan.actions.len(), deserialized.actions.len());
    }
}
