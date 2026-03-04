// projects/products/unstable/evolutionary_system_generator/backend/src/search/population.rs
use crate::search::individual::Individual;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub generation: u32,
    pub individuals: Vec<Individual>,
}
