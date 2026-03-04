// projects/products/unstable/evolutionary_system_generator/backend/src/genetics/genome_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenomeId(pub u32);
