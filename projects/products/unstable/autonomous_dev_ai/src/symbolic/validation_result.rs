// projects/products/unstable/autonomous_dev_ai/src/symbolic/validation_result.rs
use crate::value_types::ActionName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub approved_action: Option<ActionName>,
}
