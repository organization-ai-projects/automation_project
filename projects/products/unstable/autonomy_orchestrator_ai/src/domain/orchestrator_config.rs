// projects/products/unstable/autonomy_orchestrator_ai/src/domain/orchestrator_config.rs
use crate::domain::{
    BinaryInvocationSpec, DecisionContribution, DecisionReliabilityInput, DeliveryOptions,
    ExecutionPolicy, GateInputs,
};
use common_binary::{BinaryOptions, read_binary, write_binary};
use common_json::{from_str, to_string_pretty};
use common_ron::{read_ron, write_ron};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const ORCHESTRATOR_CONFIG_BIN_MAGIC: [u8; 4] = *b"AOCF";
const ORCHESTRATOR_CONFIG_BIN_SCHEMA_ID: u64 = 1;

#[derive(Debug, Deserialize)]
struct OrchestratorConfigJsonCompat {
    run_id: String,
    simulate_blocked: bool,
    planning_invocation: Option<BinaryInvocationSpec>,
    execution_invocation: Option<BinaryInvocationSpec>,
    validation_invocation: Option<BinaryInvocationSpec>,
    execution_policy: ExecutionPolicyJsonCompat,
    timeout_ms: u64,
    repo_root: PathBuf,
    planning_context_artifact: Option<PathBuf>,
    validation_invocations: Vec<BinaryInvocationSpec>,
    validation_from_planning_context: bool,
    delivery_options: DeliveryOptions,
    gate_inputs: GateInputs,
    decision_threshold: Option<f64>,
    decision_contributions: Option<Vec<DecisionContribution>>,
    decision_reliability_inputs: Option<Vec<DecisionReliabilityInput>>,
    decision_require_contributions: Option<bool>,
    checkpoint_path: Option<PathBuf>,
    cycle_memory_path: Option<PathBuf>,
    next_actions_path: Option<PathBuf>,
    previous_run_report_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct ExecutionPolicyJsonCompat {
    execution_max_iterations: f64,
    reviewer_remediation_max_cycles: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub run_id: String,
    pub simulate_blocked: bool,
    pub planning_invocation: Option<BinaryInvocationSpec>,
    pub execution_invocation: Option<BinaryInvocationSpec>,
    pub validation_invocation: Option<BinaryInvocationSpec>,
    pub execution_policy: ExecutionPolicy,
    pub timeout_ms: u64,
    pub repo_root: PathBuf,
    pub planning_context_artifact: Option<PathBuf>,
    pub validation_invocations: Vec<BinaryInvocationSpec>,
    pub validation_from_planning_context: bool,
    pub delivery_options: DeliveryOptions,
    pub gate_inputs: GateInputs,
    pub decision_threshold: u8,
    pub decision_contributions: Vec<DecisionContribution>,
    pub decision_reliability_inputs: Vec<DecisionReliabilityInput>,
    pub decision_require_contributions: bool,
    pub checkpoint_path: Option<PathBuf>,
    pub cycle_memory_path: Option<PathBuf>,
    pub next_actions_path: Option<PathBuf>,
    pub previous_run_report_path: Option<PathBuf>,
    pub planner_fallback_max_steps: u32,
}

impl OrchestratorConfig {
    fn extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.trim().to_ascii_lowercase())
    }

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

    pub fn save_json(&self, path: &Path) -> Result<(), String> {
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
        let json = to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize orchestrator config JSON: {e:?}"))?;
        fs::write(path, json).map_err(|e| {
            format!(
                "Failed to write orchestrator config JSON '{}': {}",
                path.display(),
                e
            )
        })
    }

    pub fn load_json(path: &Path) -> Result<Self, String> {
        let raw = fs::read_to_string(path).map_err(|e| {
            format!(
                "Failed to read orchestrator config JSON '{}': {}",
                path.display(),
                e
            )
        })?;
        let parsed: OrchestratorConfigJsonCompat = from_str(&raw).map_err(|e| {
            format!(
                "Failed to parse orchestrator config JSON '{}': {e:?}",
                path.display()
            )
        })?;
        Ok(Self {
            run_id: parsed.run_id,
            simulate_blocked: parsed.simulate_blocked,
            planning_invocation: parsed.planning_invocation,
            execution_invocation: parsed.execution_invocation,
            validation_invocation: parsed.validation_invocation,
            execution_policy: ExecutionPolicy {
                execution_max_iterations: float_to_u32_compat(
                    parsed.execution_policy.execution_max_iterations,
                    "execution_max_iterations",
                )?,
                reviewer_remediation_max_cycles: float_to_u32_compat(
                    parsed.execution_policy.reviewer_remediation_max_cycles,
                    "reviewer_remediation_max_cycles",
                )?,
            },
            timeout_ms: parsed.timeout_ms,
            repo_root: parsed.repo_root,
            planning_context_artifact: parsed.planning_context_artifact,
            validation_invocations: parsed.validation_invocations,
            validation_from_planning_context: parsed.validation_from_planning_context,
            delivery_options: parsed.delivery_options,
            gate_inputs: parsed.gate_inputs,
            decision_threshold: parsed
                .decision_threshold
                .map(|v| float_to_u8_compat(v, "decision_threshold"))
                .transpose()?
                .unwrap_or(70),
            decision_contributions: parsed.decision_contributions.unwrap_or_default(),
            decision_reliability_inputs: parsed.decision_reliability_inputs.unwrap_or_default(),
            decision_require_contributions: parsed.decision_require_contributions.unwrap_or(false),
            checkpoint_path: parsed.checkpoint_path,
            cycle_memory_path: parsed.cycle_memory_path,
            next_actions_path: parsed.next_actions_path,
            previous_run_report_path: parsed.previous_run_report_path,
            planner_fallback_max_steps: 3,
        })
    }

    pub fn save_auto(&self, path: &Path) -> Result<(), String> {
        match Self::extension(path).as_deref() {
            None => self.save_bin(path),
            Some("ron") => self.save_ron(path),
            Some("bin") => self.save_bin(path),
            Some("json") => self.save_json(path),
            _ => Err(format!(
                "Unsupported config extension for '{}': expected .ron, .bin, or .json (or no extension for binary)",
                path.display()
            )),
        }
    }

    pub fn load_auto(path: &Path) -> Result<Self, String> {
        match Self::extension(path).as_deref() {
            None => Self::load_bin(path),
            Some("ron") => Self::load_ron(path),
            Some("bin") => Self::load_bin(path),
            Some("json") => Self::load_json(path),
            _ => Err(format!(
                "Unsupported config extension for '{}': expected .ron, .bin, or .json (or no extension for binary)",
                path.display()
            )),
        }
    }
}

fn float_to_u32_compat(value: f64, field: &str) -> Result<u32, String> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return Err(format!(
            "Failed to parse orchestrator config JSON field '{field}': expected non-negative integer-compatible number"
        ));
    }
    let as_u64 = value as u64;
    u32::try_from(as_u64).map_err(|_| {
        format!("Failed to parse orchestrator config JSON field '{field}': value is too large")
    })
}

fn float_to_u8_compat(value: f64, field: &str) -> Result<u8, String> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return Err(format!(
            "Failed to parse orchestrator config JSON field '{field}': expected integer 0..255"
        ));
    }
    let as_u64 = value as u64;
    u8::try_from(as_u64).map_err(|_| {
        format!("Failed to parse orchestrator config JSON field '{field}': value is too large")
    })
}
