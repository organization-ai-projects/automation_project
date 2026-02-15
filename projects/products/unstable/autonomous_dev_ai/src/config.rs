// projects/products/unstable/autonomous_dev_ai/src/config.rs

use crate::error::{AgentError, AgentResult};
use crate::objectives::{Objective, default_objectives};
use common_ron::{read_ron, write_ron};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Neural configuration
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct NeuralConfig {
    pub enabled: bool,
    pub prefer_gpu: bool,
    pub cpu_fallback: bool,
    pub models: HashMap<String, String>,
}

impl Default for NeuralConfig {
    fn default() -> Self {
        let mut models = HashMap::new();
        models.insert("intent".to_string(), "intent_v1.bin".to_string());
        models.insert("codegen".to_string(), "codegen_v2.bin".to_string());
        models.insert("review".to_string(), "review_v1.bin".to_string());

        Self {
            enabled: true,
            prefer_gpu: true,
            cpu_fallback: true,
            models,
        }
    }
}

/// Symbolic configuration
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct SymbolicConfig {
    pub strict_validation: bool,
    pub deterministic: bool,
}

impl Default for SymbolicConfig {
    fn default() -> Self {
        Self {
            strict_validation: true,
            deterministic: true,
        }
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[serde(default)]
pub struct AgentConfig {
    pub agent_name: String,
    pub execution_mode: String,
    pub neural: NeuralConfig,
    pub symbolic: SymbolicConfig,
    pub objectives: Vec<Objective>,
    pub max_iterations: usize,
    pub timeout_seconds: Option<u64>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            agent_name: "autonomous_dev_ai".to_string(),
            execution_mode: "ci_bound".to_string(),
            neural: NeuralConfig::default(),
            symbolic: SymbolicConfig::default(),
            objectives: default_objectives(),
            max_iterations: 100,
            timeout_seconds: Some(3600),
        }
    }
}

/// Load configuration with fallback mechanism
/// 1. Try to load .bin file
/// 2. If missing or incompatible, load .ron file
/// 3. Rebuild .bin deterministically
pub fn load_config<P: AsRef<Path>>(base_path: P) -> AgentResult<AgentConfig> {
    let base = base_path.as_ref();
    let bin_path = base.with_extension("bin");
    let ron_path = base.with_extension("ron");

    // Try loading .bin first
    if bin_path.exists() {
        match load_bin(&bin_path) {
            Ok(config) => return Ok(config),
            Err(e) => {
                tracing::warn!("Failed to load .bin config: {}, falling back to .ron", e);
            }
        }
    }

    // Fall back to .ron
    if ron_path.exists() {
        let config = load_ron(&ron_path)?;

        // Try to rebuild .bin
        if let Err(e) = save_bin(&bin_path, &config) {
            tracing::warn!("Failed to save .bin config: {}", e);
        }

        Ok(config)
    } else {
        // No config found, use default
        let config = AgentConfig::default();

        // Save both formats
        save_ron(&ron_path, &config)?;
        save_bin(&bin_path, &config)?;

        Ok(config)
    }
}

/// Load configuration from .bin file
fn load_bin<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    let bytes = fs::read(path)?;
    bincode::decode_from_slice(&bytes, bincode::config::standard())
        .map(|(config, _)| config)
        .map_err(|e| AgentError::Bincode(format!("{:?}", e)))
}

/// Load configuration from .ron file
fn load_ron<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    read_ron(path).map_err(|e| AgentError::Ron(e.to_string()))
}

/// Save configuration to .bin file
pub fn save_bin<P: AsRef<Path>>(path: P, config: &AgentConfig) -> AgentResult<()> {
    let bytes = bincode::encode_to_vec(config, bincode::config::standard())
        .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
    fs::write(path, bytes)?;
    Ok(())
}

/// Save configuration to .ron file
pub fn save_ron<P: AsRef<Path>>(path: P, config: &AgentConfig) -> AgentResult<()> {
    write_ron(path, config).map_err(|e| AgentError::Ron(e.to_string()))
}
