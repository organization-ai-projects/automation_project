// projects/products/unstable/autonomy_orchestrator_ai/src/domain/mod.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Planning,
    PolicyIngestion,
    Execution,
    Validation,
    Closure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalState {
    Done,
    Blocked,
    Failed,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageTransition {
    pub run_id: String,
    pub from_stage: Option<Stage>,
    pub to_stage: Stage,
    pub timestamp_unix_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunReport {
    pub product: String,
    pub version: String,
    pub run_id: String,
    pub current_stage: Option<Stage>,
    pub terminal_state: Option<TerminalState>,
    pub transitions: Vec<StageTransition>,
}

impl RunReport {
    pub fn new(run_id: String) -> Self {
        Self {
            product: "autonomy_orchestrator_ai".to_string(),
            version: "0.1.0".to_string(),
            run_id,
            current_stage: None,
            terminal_state: None,
            transitions: Vec::new(),
        }
    }
}
