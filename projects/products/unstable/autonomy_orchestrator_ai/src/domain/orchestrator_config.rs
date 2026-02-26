use crate::domain::{BinaryInvocationSpec, DeliveryOptions, GateInputs};
use bincode::config::standard;
use bincode::serde::{decode_from_slice, encode_to_vec};
use ron::de::from_str as ron_from_str;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
        let text = to_string_pretty(self, PrettyConfig::new())
            .map_err(|e| format!("Failed to serialize orchestrator config RON: {e}"))?;
        fs::write(path, text).map_err(|e| {
            format!(
                "Failed to write orchestrator config RON '{}': {}",
                path.display(),
                e
            )
        })
    }

    pub fn load_ron(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|e| {
            format!(
                "Failed to read orchestrator config RON '{}': {}",
                path.display(),
                e
            )
        })?;
        ron_from_str(&text).map_err(|e| {
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
        let bytes = encode_to_vec(self, standard())
            .map_err(|e| format!("Failed to serialize orchestrator config BIN: {e}"))?;
        fs::write(path, bytes).map_err(|e| {
            format!(
                "Failed to write orchestrator config BIN '{}': {}",
                path.display(),
                e
            )
        })
    }

    pub fn load_bin(path: &Path) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| {
            format!(
                "Failed to read orchestrator config BIN '{}': {}",
                path.display(),
                e
            )
        })?;
        let (config, _): (Self, usize) = decode_from_slice(&bytes, standard()).map_err(|e| {
            format!(
                "Failed to parse orchestrator config BIN '{}': {e}",
                path.display()
            )
        })?;
        Ok(config)
    }
}
