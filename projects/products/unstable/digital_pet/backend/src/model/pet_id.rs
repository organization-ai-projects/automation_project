// projects/products/unstable/digital_pet/backend/src/model/pet_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PetId(pub u64);

impl PetId {
    pub fn from_seed(seed: u64) -> Self { Self(seed) }
}
