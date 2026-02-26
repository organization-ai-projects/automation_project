// projects/products/unstable/autonomy_orchestrator_ai/src/artifacts/orchestrator_cycle_memory.rs
use common_binary::{BinaryOptions, read_binary, write_binary};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::artifacts::{ExecutionPolicyOverrides, ValidationInvocationArtifact};

const ORCHESTRATOR_CYCLE_MEMORY_MAGIC: [u8; 4] = *b"AOCM";
const ORCHESTRATOR_CYCLE_MEMORY_SCHEMA_ID: u64 = 1;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrchestratorCycleMemory {
    pub execution_policy_overrides: ExecutionPolicyOverrides,
    pub planned_remediation_steps: Vec<String>,
    pub validation_commands: Vec<ValidationInvocationArtifact>,
    pub updated_at_unix_secs: u64,
}

fn cycle_memory_bin_options() -> BinaryOptions {
    BinaryOptions {
        magic: ORCHESTRATOR_CYCLE_MEMORY_MAGIC,
        container_version: 1,
        schema_id: ORCHESTRATOR_CYCLE_MEMORY_SCHEMA_ID,
        verify_checksum: true,
    }
}

pub fn load_cycle_memory(path: &Path) -> Result<OrchestratorCycleMemory, String> {
    read_binary(path, &cycle_memory_bin_options()).map_err(|e| {
        format!(
            "Failed to parse orchestrator cycle memory BIN '{}': {e}",
            path.display()
        )
    })
}

pub fn save_cycle_memory(path: &Path, memory: &OrchestratorCycleMemory) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create cycle memory parent dir '{}': {}",
                parent.display(),
                e
            )
        })?;
    }
    write_binary(memory, path, &cycle_memory_bin_options()).map_err(|e| {
        format!(
            "Failed to write orchestrator cycle memory BIN '{}': {}",
            path.display(),
            e
        )
    })
}
