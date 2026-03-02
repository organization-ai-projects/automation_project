use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsView {
    pub total_watch_ticks: u64,
    pub completion_rate_pct: f32,
    pub episodes_watched: usize,
}
