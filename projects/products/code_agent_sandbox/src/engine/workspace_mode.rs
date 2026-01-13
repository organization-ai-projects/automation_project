// projects/products/code_agent_sandbox/src/engine/workspace_mode.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum WorkspaceMode {
    #[default]
    Assist,
    Learn,
}
