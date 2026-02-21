// projects/products/unstable/autonomous_dev_ai/src/lifecycle/checkpoint.rs
use serde::{Deserialize, Serialize};

/// Saved checkpoint that allows the agent to resume after a crash/restart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub run_id: String,
    pub iteration: usize,
    pub state_label: String,
    pub timestamp_secs: u64,
}

impl Checkpoint {
    pub fn new(
        run_id: impl Into<String>,
        iteration: usize,
        state_label: impl Into<String>,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            iteration,
            state_label: state_label.into(),
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Persist the checkpoint to a JSON file atomically (write-then-rename).
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        let tmp = format!("{path}.tmp");
        std::fs::write(&tmp, &json)?;
        std::fs::rename(&tmp, path)
    }

    /// Load the latest checkpoint from a JSON file.
    pub fn load(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
