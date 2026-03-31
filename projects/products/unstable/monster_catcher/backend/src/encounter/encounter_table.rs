use crate::data::species_id::SpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterEntry {
    pub species_id: SpeciesId,
    pub min_level: u32,
    pub max_level: u32,
    pub weight: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EncounterTable {
    pub entries: Vec<EncounterEntry>,
}

impl EncounterTable {
    pub fn total_weight(&self) -> u32 {
        self.entries.iter().map(|e| e.weight).sum()
    }
}
