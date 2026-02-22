// projects/products/unstable/autonomy_orchestrator_ai/src/domain/run_report.rs

use serde::{Deserialize, Serialize};

use crate::domain::{Stage, StageExecutionRecord, StageTransition, TerminalState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunReport {
    pub product: String,
    pub version: String,
    pub run_id: String,
    pub current_stage: Option<Stage>,
    pub terminal_state: Option<TerminalState>,
    pub transitions: Vec<StageTransition>,
    pub stage_executions: Vec<StageExecutionRecord>,
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
            stage_executions: Vec::new(),
        }
    }
}
