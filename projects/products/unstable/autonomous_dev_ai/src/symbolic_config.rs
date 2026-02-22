//projects/products/unstable/autonomous_dev_ai/src/symbolic_config.rs
use serde::{Deserialize, Serialize};

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
