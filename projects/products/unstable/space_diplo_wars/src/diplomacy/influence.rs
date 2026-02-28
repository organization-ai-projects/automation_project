use serde::{Deserialize, Serialize};

use crate::model::empire_id::EmpireId;

/// Diplomatic influence of one empire towards another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Influence {
    pub empire: EmpireId,
    pub target: EmpireId,
    /// Positive = friendly, negative = hostile.
    pub value: i32,
}
