// projects/products/unstable/digital_pet/backend/src/evolution/evolution_rule.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRule {
    pub from_species: String,
    pub to_species: String,
    pub min_ticks: u64,
    pub max_care_mistakes: usize,
    pub min_happiness: u32,
    pub min_discipline: u32,
}
