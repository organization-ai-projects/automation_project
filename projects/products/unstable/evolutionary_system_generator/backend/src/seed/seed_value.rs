// projects/products/unstable/evolutionary_system_generator/backend/src/seed/seed_value.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedValue(pub u64);
