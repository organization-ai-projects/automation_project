//! projects/products/unstable/autonomous_dev_ai/src/lifecycle/checkpoint.rs
use crate::ids::RunId;
use crate::value_types::StateLabel;
use common_time::Timestamp;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{self, Path},
    time,
};

/// Saved checkpoint that allows the agent to resume after a crash/restart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub run_id: RunId,
    pub iteration: usize,
    pub state_label: StateLabel,
    pub timestamp_secs: Timestamp,
}

impl Checkpoint {
    pub fn new(run_id: RunId, iteration: usize, state_label: StateLabel) -> Self {
        Self {
            run_id,
            iteration,
            state_label,
            timestamp_secs: time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Persist the checkpoint to a JSON file atomically (write-then-rename).
    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let json = common_json::to_string_pretty(self).map_err(io::Error::other)?;
        let tmp = path::PathBuf::from(format!("{}.tmp", path.to_string_lossy()));
        fs::write(&tmp, &json)?;
        fs::rename(&tmp, path)
    }

    /// Load the latest checkpoint from a JSON file.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let json = fs::read_to_string(path)?;
        common_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
