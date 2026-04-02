use serde::{Deserialize, Serialize};

use crate::state::StateValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Equals { variable: String, value: StateValue },
    GreaterThan { variable: String, value: i64 },
    LessThan { variable: String, value: i64 },
}
