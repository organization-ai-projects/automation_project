//! projects/products/unstable/autonomous_dev_ai/src/persistence/config_io.rs
use std::path::Path;

use common_binary::{BinaryOptions, read_binary, write_binary};
use common_ron::{read_ron, write_ron};

use crate::{
    error::{AgentError, AgentResult},
    models::AgentConfig,
};

const BINARY_OPTIONS: BinaryOptions = BinaryOptions {
    magic: *b"AGNT",
    container_version: 1,
    schema_id: 1,
    verify_checksum: true,
};

/// Load configuration from .bin file.
pub fn load_bin<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    read_binary(path, &BINARY_OPTIONS).map_err(|e| AgentError::Serialization(format!("{:?}", e)))
}

/// Load configuration from .ron file.
pub fn load_ron<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    read_ron(path).map_err(|e| AgentError::Ron(e.to_string()))
}

/// Save configuration to .bin file.
pub fn save_bin<P: AsRef<Path>>(path: P, config: &AgentConfig) -> AgentResult<()> {
    write_binary(config, path, &BINARY_OPTIONS)
        .map_err(|e| AgentError::Serialization(format!("{:?}", e)))
}

/// Save configuration to .ron file.
pub fn save_ron<P: AsRef<Path>>(path: P, config: &AgentConfig) -> AgentResult<()> {
    write_ron(path, config).map_err(|e| AgentError::Ron(e.to_string()))
}
