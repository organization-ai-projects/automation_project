// projects/products/code_agent_sandbox/src/agents/agent_request.rs
use serde::Deserialize;
use std::path;

use crate::agents::default_max_iters;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentRequest {
    /// The human intent (nocode): "add an endpoint", "refactor this module", etc.
    pub(crate) intent: String,

    /// Max iterations
    #[serde(default = "default_max_iters")]
    pub(crate) max_iters: usize,

    /// Strategy if you want to force it (otherwise the dispatcher decides)
    #[serde(default)]
    pub(crate) forced_strategy: Option<String>,

    /// Optional: the main file to target (otherwise the AI can manage by reading src/)
    #[serde(default)]
    pub(crate) focus_file: Option<String>,

    /// Directory for models (e.g., ./models)
    #[serde(default)]
    pub(crate) model_dir: Option<path::PathBuf>,

    /// File for the replay buffer (e.g., ./replay.jsonl)
    #[serde(default)]
    pub(crate) replay_path: Option<path::PathBuf>,
}
