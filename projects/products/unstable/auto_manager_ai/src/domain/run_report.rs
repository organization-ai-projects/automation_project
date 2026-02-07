// projects/products/unstable/auto_manager_ai/src/domain/run_report.rs

use super::policy_decision::PolicyDecision;
use super::policy_decision_type::PolicyDecisionType;
use super::run_output::RunOutput;
use super::run_status::RunStatus;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use crate::domain::policy_decision::PolicyDecision;
    use crate::domain::policy_decision_type::PolicyDecisionType;
    use crate::domain::run_report::RunReport;
    use crate::domain::run_status::RunStatus;
    use common_json::{JsonAccess, from_str, to_string_pretty};

    #[test]
    fn test_run_report_new() {
        let report = RunReport::new("test_run_123".to_string());
        assert_eq!(report.product, "auto_manager_ai");
        assert_eq!(report.version, "0.1.0");
        assert_eq!(report.run_id, "test_run_123");
        assert_eq!(report.status, RunStatus::Success);
        assert_eq!(report.errors.len(), 0);
    }

    #[test]
    fn test_run_report_serialization() {
        let report = RunReport::new("test_run_123".to_string());

        let json = to_string_pretty(&report).expect("Failed to serialize");
        let parsed: common_json::Json = from_str(&json).expect("Failed to parse JSON");

        // Verify specific fields exist and have correct values
        let product = parsed
            .get_field("product")
            .expect("product field should exist");
        assert!(
            matches!(product, common_json::Json::String(_)),
            "product should be a string"
        );
        assert_eq!(
            product.as_str(),
            Some("auto_manager_ai"),
            "product should match"
        );

        let version = parsed
            .get_field("version")
            .expect("version field should exist");
        assert!(
            matches!(version, common_json::Json::String(_)),
            "version should be a string"
        );
        assert_eq!(version.as_str(), Some("0.1.0"), "version should be 0.1.0");

        let run_id = parsed
            .get_field("run_id")
            .expect("run_id field should exist");
        assert!(
            matches!(run_id, common_json::Json::String(_)),
            "run_id should be a string"
        );
        assert_eq!(run_id.as_str(), Some("test_run_123"), "run_id should match");

        let timestamp = parsed
            .get_field("timestamp")
            .expect("timestamp field should exist");
        assert!(
            matches!(timestamp, common_json::Json::String(_)),
            "timestamp should be a string"
        );

        let status = parsed
            .get_field("status")
            .expect("status field should exist");
        assert!(
            matches!(status, common_json::Json::String(_)),
            "status should be a string"
        );

        let output = parsed
            .get_field("output")
            .expect("output field should exist");
        assert!(
            matches!(output, common_json::Json::Object(_)),
            "output should be an object"
        );

        let policy_decisions = parsed
            .get_field("policy_decisions")
            .expect("policy_decisions field should exist");
        assert!(
            matches!(policy_decisions, common_json::Json::Array(_)),
            "policy_decisions should be an array"
        );

        let errors = parsed
            .get_field("errors")
            .expect("errors field should exist");
        assert!(
            matches!(errors, common_json::Json::Array(_)),
            "errors should be an array"
        );

        let _deserialized: RunReport = from_str(&json).expect("Failed to deserialize");
    }

    #[test]
    fn test_run_report_add_error() {
        let mut report = RunReport::new("test".to_string());
        assert_eq!(report.status, RunStatus::Success);

        report.add_error("Test error".to_string());

        assert_eq!(report.status, RunStatus::Failure);
        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.errors[0], "Test error");
    }

    #[test]
    fn test_run_report_add_decision() {
        let mut report = RunReport::new("test".to_string());

        let decision = PolicyDecision {
            action_id: "action_001".to_string(),
            decision: PolicyDecisionType::Allow,
            reason: "Test".to_string(),
        };

        report.add_decision(decision);

        assert_eq!(report.output.actions_allowed, 1);
        assert_eq!(report.policy_decisions.len(), 1);
    }
}
