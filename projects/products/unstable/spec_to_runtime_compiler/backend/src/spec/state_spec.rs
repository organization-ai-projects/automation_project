use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpec {
    pub name: String,
    pub fields: Vec<crate::spec::spec::FieldSpec>,
}
