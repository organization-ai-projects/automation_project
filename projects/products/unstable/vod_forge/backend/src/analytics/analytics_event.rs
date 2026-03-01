use serde::{Deserialize, Serialize};
use crate::playback::profile_id::ProfileId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub profile_id: ProfileId,
    pub episode_id: String,
    pub ticks_watched: u32,
    pub completed: bool,
    pub at_tick: u32,
}
