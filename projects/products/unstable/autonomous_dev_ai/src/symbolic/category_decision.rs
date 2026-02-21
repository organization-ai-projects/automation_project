use serde::{Deserialize, Serialize};

use crate::symbolic::{CategorySource, IssueCategory};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDecision {
    pub category: IssueCategory,
    pub source: CategorySource,
    pub confidence: f64,
}
