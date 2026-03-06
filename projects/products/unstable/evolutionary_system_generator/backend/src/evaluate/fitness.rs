// projects/products/unstable/evolutionary_system_generator/backend/src/evaluate/fitness.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Fitness(pub f64);
