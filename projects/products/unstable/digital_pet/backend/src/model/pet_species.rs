// projects/products/unstable/digital_pet/backend/src/model/pet_species.rs
use crate::model::pet_species_id::PetSpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetSpecies {
    pub id: PetSpeciesId,
    pub name: String,
    pub base_attack: u32,
    pub base_defense: u32,
    pub base_hp: u32,
}

impl PetSpecies {
    pub fn egg() -> Self {
        Self {
            id: PetSpeciesId::new("egg"),
            name: "Egg".to_string(),
            base_attack: 1,
            base_defense: 1,
            base_hp: 10,
        }
    }
}
