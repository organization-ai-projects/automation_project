//projects/products/unstable/autonomous_dev_ai/src/ops/run_replay.rs
use serde::{Deserialize, Serialize};

use crate::ops::ReplayEvent;

/// Replay log that can reconstruct the full causal run timeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunReplay {
    pub run_id: String,
    pub events: Vec<ReplayEvent>,
}

impl RunReplay {
    pub fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            events: Vec::new(),
        }
    }

    pub fn record(&mut self, kind: &str, payload: impl Into<String>) {
        let sequence = self.events.len();
        self.events.push(ReplayEvent {
            sequence,
            kind: kind.to_string(),
            payload: payload.into(),
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
    }

    /// Reconstruct timeline as a human-readable string.
    pub fn reconstruct(&self) -> String {
        let mut lines = vec![format!("=== Run Replay: {} ===", self.run_id)];
        for ev in &self.events {
            lines.push(format!(
                "[{}] #{} {} — {}",
                ev.timestamp_secs, ev.sequence, ev.kind, ev.payload
            ));
        }
        lines.join("\n")
    }

    /// Write replay to a JSON file.
    pub fn persist(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }
}
