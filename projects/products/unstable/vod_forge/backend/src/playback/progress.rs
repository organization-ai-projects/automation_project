use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub episode_id: String,
    pub tick: u32,
    pub duration_ticks: u32,
}

impl Progress {
    pub fn progress_pct(&self) -> f32 {
        if self.duration_ticks == 0 {
            return 100.0;
        }
        (self.tick as f32 / self.duration_ticks as f32) * 100.0
    }
}
