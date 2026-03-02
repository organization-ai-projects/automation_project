#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// View of a completed run report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportScreen {
    pub run_hash: String,
    pub visitors_served: u32,
    pub total_revenue: u64,
    pub final_reputation: i32,
    pub event_count: usize,
}

impl ReportScreen {
    pub fn render(&self) -> String {
        format!(
            "[Report] hash={} served={} revenue={} reputation={} events={}",
            &self.run_hash[..8],
            self.visitors_served,
            self.total_revenue,
            self.final_reputation,
            self.event_count
        )
    }
}
