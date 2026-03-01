use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub tick: Tick,
    pub description: String,
    pub sentiment: i32,
}
