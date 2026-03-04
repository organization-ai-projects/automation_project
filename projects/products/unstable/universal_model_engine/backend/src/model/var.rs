use crate::model::domain::Domain;
use crate::model::var_id::VarId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Var {
    pub id: VarId,
    pub domain: Domain,
    pub initial_value: i64,
}

impl Var {
    pub fn integer(name: &str, initial_value: i64) -> Self {
        Self {
            id: VarId(name.to_string()),
            domain: Domain::Integer,
            initial_value,
        }
    }
}
