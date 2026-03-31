use serde::{Deserialize, Serialize};

use crate::state::StateValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Set { variable: String, value: StateValue },
    Add { variable: String, amount: i64 },
    Log { message: String },
}
