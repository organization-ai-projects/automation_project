// projects/products/unstable/autonomy_orchestrator_ai/src/auto_fix_loop.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoFixLoopConfig {
    pub enabled: bool,
    pub max_attempts: u32,
}

impl Default for AutoFixLoopConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_attempts: 3,
        }
    }
}
