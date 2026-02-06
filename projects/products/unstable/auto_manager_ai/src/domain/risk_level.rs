// projects/products/unstable/auto_manager_ai/src/domain/risk_level.rs

use serde::{Deserialize, Serialize};

/// Risk level for an action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}
