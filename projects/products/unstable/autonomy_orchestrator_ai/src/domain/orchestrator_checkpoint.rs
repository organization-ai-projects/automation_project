// projects/products/unstable/autonomy_orchestrator_ai/src/domain/orchestrator_checkpoint.rs
use serde::{Deserialize, Serialize};

use crate::domain::{Stage, TerminalState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestratorCheckpoint {
    pub run_id: String,
    pub completed_stages: Vec<Stage>,
    pub terminal_state: Option<TerminalState>,
    pub updated_at_unix_secs: u64,
}

impl OrchestratorCheckpoint {
    pub fn new(run_id: String, updated_at_unix_secs: u64) -> Self {
        Self {
            run_id,
            completed_stages: Vec::new(),
            terminal_state: None,
            updated_at_unix_secs,
        }
    }

    pub fn is_stage_completed(&self, stage: Stage) -> bool {
        self.completed_stages.contains(&stage)
    }

    pub fn mark_stage_completed(&mut self, stage: Stage, updated_at_unix_secs: u64) {
        if !self.is_stage_completed(stage) {
            self.completed_stages.push(stage);
        }
        self.updated_at_unix_secs = updated_at_unix_secs;
    }

    pub fn mark_terminal(&mut self, terminal_state: TerminalState, updated_at_unix_secs: u64) {
        self.terminal_state = Some(terminal_state);
        self.updated_at_unix_secs = updated_at_unix_secs;
    }
}
