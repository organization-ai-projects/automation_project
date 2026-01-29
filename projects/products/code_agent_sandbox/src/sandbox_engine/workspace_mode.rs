// projects/products/code_agent_sandbox/src/engine/workspace_mode.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkspaceMode {
    #[default]
    Assist,
    Learn,
}
