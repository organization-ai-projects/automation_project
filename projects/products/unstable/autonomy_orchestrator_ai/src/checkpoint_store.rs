// projects/products/unstable/autonomy_orchestrator_ai/src/checkpoint_store.rs

use crate::domain::OrchestratorCheckpoint;
use common_json::{from_str, to_string_pretty};
use std::fs;
use std::path::Path;

pub fn load_checkpoint(path: &Path) -> Result<OrchestratorCheckpoint, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read checkpoint '{}': {}", path.display(), e))?;
    from_str(&raw).map_err(|e| format!("Failed to parse checkpoint '{}': {:?}", path.display(), e))
}

pub fn save_checkpoint(path: &Path, checkpoint: &OrchestratorCheckpoint) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create checkpoint parent dir '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    let json = to_string_pretty(checkpoint)
        .map_err(|e| format!("Failed to serialize checkpoint: {:?}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write checkpoint '{}': {}", path.display(), e))
}
