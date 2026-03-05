use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetStateDto {
    pub species: String,
    pub evolution_stage: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub tick: u64,
}
