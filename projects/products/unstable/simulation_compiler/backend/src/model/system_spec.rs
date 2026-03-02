// projects/products/unstable/simulation_compiler/backend/src/model/system_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSpec {
    pub name: String,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
}
