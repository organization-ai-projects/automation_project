// projects/products/unstable/autonomous_dev_ai/src/persistence.rs
use std::fs;
use std::path::Path;

use common_ron::{read_ron, write_ron};

use crate::agent_config::AgentConfig;
use crate::error::{AgentError, AgentResult};
// Load configuration from .bin file
pub fn load_bin<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    let bytes = fs::read(path)?;
    bincode::decode_from_slice(&bytes, bincode::config::standard())
        .map(|(config, _)| config)
        .map_err(|e| AgentError::Bincode(format!("{:?}", e)))
}

/// Load configuration from .ron file
pub fn load_ron<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
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
