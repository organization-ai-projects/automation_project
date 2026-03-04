// projects/products/unstable/evolutionary_system_generator/backend/src/seed/seed.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed(pub u64);
