use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPolicy {
    pub max_entries: u32,
    pub decay_window_runs: u32,
}

impl Default for MemoryPolicy {
    fn default() -> Self {
        Self {
            max_entries: 500,
            decay_window_runs: 100,
        }
    }
}
