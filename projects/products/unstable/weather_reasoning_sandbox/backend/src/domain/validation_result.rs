use serde::{Deserialize, Serialize};

use crate::domain::constraint_violation::ConstraintViolation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub violations: Vec<ConstraintViolation>,
    pub is_coherent: bool,
}

impl ValidationResult {
    pub fn coherent() -> Self {
        Self {
            violations: Vec::new(),
            is_coherent: true,
        }
    }

    pub fn with_violations(violations: Vec<ConstraintViolation>) -> Self {
        let is_coherent = violations.is_empty();
        Self {
            violations,
            is_coherent,
        }
    }
}
