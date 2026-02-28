use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintReport {
    pub satisfied: Vec<String>,
    pub violated: Vec<String>,
}
