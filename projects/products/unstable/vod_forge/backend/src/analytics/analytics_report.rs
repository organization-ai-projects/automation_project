use crate::analytics::analytics_log::AnalyticsLog;
use crate::playback::profile_id::ProfileId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub profile_id: ProfileId,
    pub total_watch_ticks: u64,
    pub completion_rate_pct: f32,
    pub episodes_watched: usize,
}

impl AnalyticsReport {
    pub fn from_log(log: &AnalyticsLog, profile_id: &str) -> Self {
        let mut events: Vec<_> = log
            .events
            .iter()
            .filter(|e| e.profile_id.0 == profile_id)
            .collect();

        // Sort for determinism
        events.sort_by(|a, b| {
            a.episode_id
                .cmp(&b.episode_id)
                .then(a.at_tick.cmp(&b.at_tick))
        });

        let total_watch_ticks: u64 = events.iter().map(|e| e.ticks_watched as u64).sum();
        let episodes_watched = events.len();
        let completed_count = events.iter().filter(|e| e.completed).count();
        let completion_rate_pct = if episodes_watched == 0 {
            0.0
        } else {
            (completed_count as f32 / episodes_watched as f32) * 100.0
        };

        AnalyticsReport {
            profile_id: ProfileId(profile_id.to_string()),
            total_watch_ticks,
            completion_rate_pct,
            episodes_watched,
        }
    }
}
