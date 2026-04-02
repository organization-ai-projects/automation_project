use serde::{Deserialize, Serialize};

use crate::domain::constraint_rule::ConstraintRule;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectionAction {
    pub triggered_by: ConstraintRule,
    pub field: String,
    pub original_value: f64,
    pub corrected_value: f64,
    pub reason: String,
}
