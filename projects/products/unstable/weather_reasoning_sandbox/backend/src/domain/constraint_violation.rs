use serde::{Deserialize, Serialize};

use crate::domain::constraint_rule::ConstraintRule;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintViolation {
    pub rule: ConstraintRule,
    pub description: String,
    pub reason: String,
    pub severity: f64,
}
