// projects/products/unstable/autonomous_dev_ai/src/lifecycle/learning_context.rs
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct LearningContext {
    pub previous_failures: usize,
    pub previous_max_iteration: usize,
    pub top_failure_kind: String,
    pub top_failure_tool: String,
    pub top_decision_action: String,
    pub recent_avg_failures: f64,
    pub recent_top_failure_kind: String,
    pub recent_top_failure_kind_confidence: f64,
    pub worst_action_outcome: String,
}

impl LearningContext {
    pub fn from_metadata(metadata: &HashMap<String, String>) -> Self {
        Self {
            previous_failures: metadata
                .get("previous_state_failures_count")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0),
            previous_max_iteration: metadata
                .get("previous_state_max_iteration")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0),
            top_failure_kind: metadata
                .get("previous_state_top_failure_kind")
                .cloned()
                .unwrap_or_default(),
            top_failure_tool: metadata
                .get("previous_state_top_failure_tool")
                .cloned()
                .unwrap_or_default(),
            top_decision_action: metadata
                .get("previous_state_top_decision_action")
                .cloned()
                .unwrap_or_default(),
            recent_avg_failures: metadata
                .get("previous_recent_avg_failures")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0),
            recent_top_failure_kind: metadata
                .get("previous_recent_top_failure_kind")
                .cloned()
                .unwrap_or_default(),
            recent_top_failure_kind_confidence: metadata
                .get("previous_recent_top_failure_kind_confidence")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0),
            worst_action_outcome: metadata
                .get("previous_state_worst_action_outcome")
                .cloned()
                .unwrap_or_default(),
        }
    }
}
