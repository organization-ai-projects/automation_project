#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Summary view of the overall park state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkScreen {
    pub tick: u64,
    pub active_visitors: usize,
    pub reputation: i32,
    pub total_revenue: u64,
}

impl ParkScreen {
    pub fn render(&self) -> String {
        format!(
            "[Park] tick={} visitors={} reputation={} revenue={}",
            self.tick, self.active_visitors, self.reputation, self.total_revenue
        )
    }
}
