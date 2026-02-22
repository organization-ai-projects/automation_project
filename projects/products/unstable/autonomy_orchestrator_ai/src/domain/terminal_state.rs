// projects/products/unstable/autonomy_orchestrator_ai/src/domain/terminal_state.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalState {
    Done,
    Blocked,
    Failed,
    Timeout,
}
