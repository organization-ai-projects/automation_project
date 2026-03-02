// projects/products/unstable/digital_pet/backend/src/scenario/scenario.rs
use crate::config::sim_config::SimConfig;
use crate::model::pet_species::PetSpecies;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub starting_species: PetSpecies,
    pub config: SimConfig,
}

impl Default for Scenario {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            starting_species: PetSpecies::egg(),
            config: SimConfig::default(),
        }
    }
}
