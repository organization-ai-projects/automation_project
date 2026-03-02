// projects/products/unstable/digital_pet/backend/src/model/pet.rs
use crate::model::pet_id::PetId;
use crate::model::pet_species::PetSpecies;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub id: PetId,
    pub name: String,
    pub species: PetSpecies,
    pub attack: u32,
    pub defense: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub weight: u32,
    pub age_ticks: u64,
    pub evolution_stage: u32,
}

impl Pet {
    pub fn new(seed: u64, species: PetSpecies) -> Self {
        Self {
            id: PetId::from_seed(seed),
            name: format!("Pet#{}", seed % 1000),
            attack: species.base_attack,
            defense: species.base_defense,
            hp: species.base_hp,
            max_hp: species.base_hp,
            weight: 10,
            age_ticks: 0,
            evolution_stage: 0,
            species,
        }
    }
    pub fn evolve_to(&mut self, new_species: PetSpecies) {
        self.attack = new_species.base_attack;
        self.defense = new_species.base_defense;
        self.max_hp = new_species.base_hp;
        self.hp = new_species.base_hp;
        self.evolution_stage += 1;
        self.species = new_species;
    }
}
