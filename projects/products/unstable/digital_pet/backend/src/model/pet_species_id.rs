// projects/products/unstable/digital_pet/backend/src/model/pet_species_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PetSpeciesId(pub String);

impl PetSpeciesId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }
}
