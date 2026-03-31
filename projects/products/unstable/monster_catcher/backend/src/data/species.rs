use crate::data::move_id::MoveId;
use crate::data::species_id::SpeciesId;
use crate::data::type_id::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub primary_type: TypeId,
    pub secondary_type: Option<TypeId>,
    pub base_hp: u32,
    pub base_attack: u32,
    pub base_defense: u32,
    pub base_speed: u32,
    pub capture_rate: u32,
    pub base_xp_yield: u32,
    pub learnset: Vec<LearnsetEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LearnsetEntry {
    pub level: u32,
    pub move_id: MoveId,
}
