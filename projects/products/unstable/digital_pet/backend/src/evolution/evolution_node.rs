// projects/products/unstable/digital_pet/backend/src/evolution/evolution_node.rs
use crate::model::pet_species::PetSpecies;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionNode {
    pub species: PetSpecies,
    pub children: Vec<EvolutionNode>,
}

impl EvolutionNode {
    pub fn new(species: PetSpecies) -> Self {
        Self {
            species,
            children: vec![],
        }
    }
    pub fn with_children(mut self, children: Vec<EvolutionNode>) -> Self {
        self.children = children;
        self
    }
}
