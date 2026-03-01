#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// View of ride states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RideScreen {
    pub ride_id: u32,
    pub kind: String,
    pub queue_len: usize,
    pub running: bool,
    pub total_riders: u32,
}

impl RideScreen {
    pub fn render(&self) -> String {
        format!(
            "[Ride {}] kind={} queue={} running={} total_riders={}",
            self.ride_id, self.kind, self.queue_len, self.running, self.total_riders
        )
    }
}
