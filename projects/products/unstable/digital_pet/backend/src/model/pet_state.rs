// projects/products/unstable/digital_pet/backend/src/model/pet_state.rs
use crate::model::pet::Pet;
use crate::needs::needs_state::NeedsState;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetState {
    pub name: String,
    pub species: String,
    pub evolution_stage: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub tick: u64,
}

impl PetState {
    pub fn from_pet_and_needs(pet: &Pet, needs: &NeedsState, tick: Tick) -> Self {
        Self {
            name: pet.name.clone(),
            species: pet.species.name.clone(),
            evolution_stage: pet.evolution_stage,
            hp: pet.hp,
            max_hp: pet.max_hp,
            attack: pet.attack,
            defense: pet.defense,
            hunger: needs.hunger,
            fatigue: needs.fatigue,
            happiness: needs.happiness,
            discipline: needs.discipline,
            sick: needs.sick,
            tick: tick.value(),
        }
    }
}
