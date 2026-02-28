// projects/products/unstable/simulation_compiler/backend/src/model/event_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSpec {
    pub name: String,
    pub fields: Vec<crate::model::component_spec::FieldSpec>,
}
