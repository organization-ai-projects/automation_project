use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSpec {
    pub from: String,
    pub to: String,
    pub event: String,
    pub guard_fields: Vec<crate::spec::spec::FieldSpec>,
}
