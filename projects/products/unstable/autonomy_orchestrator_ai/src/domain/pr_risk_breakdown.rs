// projects/products/unstable/autonomy_orchestrator_ai/src/domain/pr_risk_breakdown.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrRiskFactor {
    pub name: String,
    pub score: u16,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrRiskBreakdown {
    pub total_score: u16,
    pub factors: Vec<PrRiskFactor>,
    pub eligible_for_auto_merge: bool,
}
