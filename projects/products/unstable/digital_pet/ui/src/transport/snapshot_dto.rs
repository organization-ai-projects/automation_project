use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDto {
    pub tick: u64,
    pub pet_name: String,
    pub species: String,
    pub evolution_stage: u32,
    pub hp: u32,
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub event_count: usize,
    pub hash: String,
}
