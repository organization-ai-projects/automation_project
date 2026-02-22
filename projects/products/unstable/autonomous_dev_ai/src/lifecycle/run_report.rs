use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub generated_at_secs: u64,
    pub run_id: String,
    pub final_state: String,
    pub total_iterations: usize,
    pub max_iterations: usize,
    pub total_decisions: usize,
    pub total_failures: usize,
    pub total_objective_evaluations: usize,
    pub last_objective_passed: Option<bool>,
    pub weighted_objective_score: Option<f64>,
    pub run_replay_path: Option<String>,
    pub run_replay_text_path: Option<String>,
    pub last_tool_failure_class: Option<String>,
    pub review_required: bool,
    pub create_pr_enabled: bool,
    pub pr_number: Option<u64>,
    pub pr_readiness: Option<String>,
    pub issue_compliance: Option<String>,
    pub pr_description_source: Option<String>,
    pub last_failure_description: Option<String>,
    pub last_failure_error: Option<String>,
    pub last_tool_exit_code: Option<i32>,
}

impl RunReport {
    pub fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
