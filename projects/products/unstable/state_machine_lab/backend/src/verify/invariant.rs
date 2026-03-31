use crate::model::state_id::StateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Invariant {
    NoDeadlock,
    StateReachable(StateId),
    VariableBound { var: String, min: i64, max: i64 },
}
