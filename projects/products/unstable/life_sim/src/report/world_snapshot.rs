use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: Tick,
    pub agent_count: usize,
    pub room_count: usize,
    pub object_count: usize,
}
