use crate::playback::profile_id::ProfileId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub profile_id: ProfileId,
    pub episode_id: String,
    pub ticks_watched: u32,
    pub completed: bool,
    pub at_tick: u32,
}
