// projects/products/unstable/autonomous_dev_ai/src/lifecycle/checkpoint.rs
use crate::ids::RunId;
use crate::value_types::StateLabel;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Saved checkpoint that allows the agent to resume after a crash/restart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub run_id: RunId,
    pub iteration: usize,
    pub state_label: StateLabel,
    pub timestamp_secs: u64,
}

impl Checkpoint {
    pub fn new(run_id: RunId, iteration: usize, state_label: StateLabel) -> Self {
        Self {
            run_id,
            iteration,
            state_label,
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Persist the checkpoint to a JSON file atomically (write-then-rename).
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        let tmp = std::path::PathBuf::from(format!("{}.tmp", path.to_string_lossy()));
        std::fs::write(&tmp, &json)?;
        std::fs::rename(&tmp, path)
    }

    /// Load the latest checkpoint from a JSON file.
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
