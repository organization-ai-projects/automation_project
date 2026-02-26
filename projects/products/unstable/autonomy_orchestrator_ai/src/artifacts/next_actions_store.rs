// projects/products/unstable/autonomy_orchestrator_ai/src/artifacts/next_actions_store.rs
use crate::domain::TerminalState;
use crate::versioning::RepoVersioningDelta;
use common_binary::{BinaryOptions, read_binary, write_binary};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const ORCHESTRATOR_NEXT_ACTIONS_MAGIC: [u8; 4] = *b"AONA";
const ORCHESTRATOR_NEXT_ACTIONS_SCHEMA_ID: u64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextActionsArtifact {
    pub run_id: String,
    pub terminal_state: Option<TerminalState>,
    pub blocked_reason_codes: Vec<String>,
    pub reviewer_next_steps: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub versioning_delta: Option<RepoVersioningDelta>,
    pub generated_at_unix_secs: u64,
}

fn next_actions_bin_options() -> BinaryOptions {
    BinaryOptions {
        magic: ORCHESTRATOR_NEXT_ACTIONS_MAGIC,
        container_version: 1,
        schema_id: ORCHESTRATOR_NEXT_ACTIONS_SCHEMA_ID,
        verify_checksum: true,
    }
}

pub fn load_next_actions(path: &Path) -> Result<NextActionsArtifact, String> {
    read_binary(path, &next_actions_bin_options()).map_err(|e| {
        format!(
            "Failed to parse orchestrator next actions BIN '{}': {e}",
            path.display()
        )
    })
}

pub fn save_next_actions(path: &Path, artifact: &NextActionsArtifact) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create next actions parent dir '{}': {}",
                parent.display(),
                e
            )
        })?;
    }
    write_binary(artifact, path, &next_actions_bin_options()).map_err(|e| {
        format!(
            "Failed to write orchestrator next actions BIN '{}': {}",
            path.display(),
            e
        )
    })
}
