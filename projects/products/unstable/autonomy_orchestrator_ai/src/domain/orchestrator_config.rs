use crate::domain::{BinaryInvocationSpec, DeliveryOptions, GateInputs};
use common_binary::{BinaryOptions, read_binary, write_binary};
use common_ron::{read_ron, write_ron};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const ORCHESTRATOR_CONFIG_BIN_MAGIC: [u8; 4] = *b"AOCF";
const ORCHESTRATOR_CONFIG_BIN_SCHEMA_ID: u64 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub run_id: String,
    pub simulate_blocked: bool,
    pub planning_invocation: Option<BinaryInvocationSpec>,
    pub execution_invocation: Option<BinaryInvocationSpec>,
    pub validation_invocation: Option<BinaryInvocationSpec>,
    pub execution_max_iterations: u32,
    pub reviewer_remediation_max_cycles: u32,
    pub timeout_ms: u64,
    pub repo_root: PathBuf,
    pub planning_context_artifact: Option<PathBuf>,
    pub validation_invocations: Vec<BinaryInvocationSpec>,
    pub validation_from_planning_context: bool,
    pub delivery_options: DeliveryOptions,
    pub gate_inputs: GateInputs,
    pub checkpoint_path: Option<PathBuf>,
}

impl OrchestratorConfig {
    fn bin_options() -> BinaryOptions {
        BinaryOptions {
            magic: ORCHESTRATOR_CONFIG_BIN_MAGIC,
            container_version: 1,
            schema_id: ORCHESTRATOR_CONFIG_BIN_SCHEMA_ID,
            verify_checksum: true,
        }
    }

    pub fn save_ron(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create config parent dir '{}': {}",
                    parent.display(),
                    e
                )
            })?;
        }
        write_ron(path, self).map_err(|e| {
            format!(
                "Failed to write orchestrator config RON '{}': {}",
                path.display(),
                e
            )
        })
    }

    pub fn load_ron(path: &Path) -> Result<Self, String> {
        read_ron(path).map_err(|e| {
            format!(
                "Failed to parse orchestrator config RON '{}': {}",
                path.display(),
                e
            )
        })
    }

    pub fn save_bin(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create config parent dir '{}': {}",
                    parent.display(),
                    e
                )
            })?;
        }
        write_binary(self, path, &Self::bin_options()).map_err(|e| {
            format!(
                "Failed to write orchestrator config BIN '{}': {}",
                path.display(),
                e
            )
        })
    }

    pub fn load_bin(path: &Path) -> Result<Self, String> {
        read_binary(path, &Self::bin_options()).map_err(|e| {
            format!(
                "Failed to parse orchestrator config BIN '{}': {e}",
                path.display()
            )
        })
    }
}
