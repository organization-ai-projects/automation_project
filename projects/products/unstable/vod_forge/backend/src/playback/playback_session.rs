use crate::playback::profile_id::ProfileId;
use crate::playback::session_id::SessionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackSession {
    pub id: SessionId,
    pub profile: ProfileId,
    pub episode_id: String,
    pub tick: u32,
    pub duration_ticks: u32,
}

impl PlaybackSession {
    pub fn is_done(&self) -> bool {
        self.tick >= self.duration_ticks
    }

    pub fn step(&mut self, steps: u32) {
        self.tick = (self.tick + steps).min(self.duration_ticks);
    }

    pub fn progress_pct(&self) -> f32 {
        if self.duration_ticks == 0 {
            return 100.0;
        }
        (self.tick as f32 / self.duration_ticks as f32) * 100.0
    }
}
