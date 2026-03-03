use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackView {
    pub session_id: String,
    pub tick: u32,
    pub progress_pct: f32,
    pub done: bool,
}
