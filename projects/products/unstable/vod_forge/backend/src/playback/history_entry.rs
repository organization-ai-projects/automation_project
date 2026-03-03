// projects/products/unstable/vod_forge/backend/src/playback/history_entry.rs
use crate::playback::ProfileId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub profile_id: ProfileId,
    pub episode_id: String,
    pub completed: bool,
    pub ticks_watched: u32,
}
