// projects/products/unstable/autonomy_orchestrator_ai/src/domain/stage_execution_record.rs

use serde::{Deserialize, Serialize};

use crate::domain::{Stage, StageExecutionStatus};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExecutionRecord {
    pub stage: Stage,
    pub idempotency_key: Option<String>,
    pub command: String,
    pub args: Vec<String>,
    pub env_keys: Vec<String>,
    pub started_at_unix_secs: u64,
    pub duration_ms: u64,
    pub exit_code: Option<i32>,
    pub status: StageExecutionStatus,
    pub error: Option<String>,
    pub missing_artifacts: Vec<String>,
    pub malformed_artifacts: Vec<String>,
}
