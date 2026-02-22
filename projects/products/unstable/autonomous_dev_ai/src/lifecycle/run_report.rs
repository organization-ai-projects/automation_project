use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub generated_at_secs: u64,
    pub run_id: String,
    pub final_state: String,
    pub execution_mode: String,
    pub neural_enabled: bool,
    pub total_iterations: usize,
    pub max_iterations: usize,
    pub total_decisions: usize,
    pub total_failures: usize,
    pub total_objective_evaluations: usize,
    pub explored_files_count: usize,
    pub last_objective_passed: Option<bool>,
    pub weighted_objective_score: Option<f64>,
    pub run_replay_path: Option<String>,
    pub run_replay_text_path: Option<String>,
    pub last_tool_failure_class: Option<String>,
    pub review_required: bool,
    pub create_pr_enabled: bool,
    pub real_pr_created: bool,
    pub pr_number: Option<u64>,
    pub pr_number_source: Option<String>,
    pub pr_ci_status: Option<String>,
    pub pr_readiness: Option<String>,
    pub issue_compliance: Option<String>,
    pub issue_context_source: Option<String>,
    pub pr_description_source: Option<String>,
    pub last_review_outcome: Option<String>,
    pub last_review_input_source: Option<String>,
    pub last_failure_description: Option<String>,
    pub last_failure_error: Option<String>,
    pub last_failure_recovery_action: Option<String>,
    pub failure_kind_counts: HashMap<String, usize>,
    pub top_failure_kind: Option<String>,
    pub last_tool_exit_code: Option<i32>,
    pub last_tool_name: Option<String>,
    pub policy_pack_fingerprint: Option<String>,
    pub checkpoint_path: Option<String>,
    pub state_transitions_total: usize,
    pub tool_executions_total: usize,
    pub tool_executions_failed: usize,
    pub risk_gate_allows: usize,
    pub risk_gate_denies: usize,
    pub risk_gate_high_approvals: usize,
    pub authz_denials_total: usize,
    pub policy_violations_total: usize,
}

impl RunReport {
    pub fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
