// projects/products/unstable/autonomy_orchestrator_ai/src/domain/auto_fix_attempt.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoFixAttemptStatus {
    Applied,
    Failed,
    SpawnFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoFixAttempt {
    pub attempt_number: u32,
    pub autofix_bin: String,
    pub exit_code: Option<i32>,
    pub status: AutoFixAttemptStatus,
    pub started_at_unix_secs: u64,
    pub duration_ms: u64,
    pub reason_code: String,
}
