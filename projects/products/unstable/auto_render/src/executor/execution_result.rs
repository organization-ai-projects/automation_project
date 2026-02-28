use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub plan_id: String,
    pub actions_applied: usize,
    pub fingerprint: String,
    pub elapsed_ms: u64,
}
