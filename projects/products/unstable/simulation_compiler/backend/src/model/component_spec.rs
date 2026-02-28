// projects/products/unstable/simulation_compiler/backend/src/model/component_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpec {
    pub name: String,
    pub fields: Vec<FieldSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    pub name: String,
    pub ty: String,
}
