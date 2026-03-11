use crate::evaluation_engine::EvaluationEngine;
use crate::orchestrator::{ContinuousGovernancePolicy, ContinuousImprovementReport};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceState {
    pub continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    pub evaluation_baseline: Option<EvaluationEngine>,
    pub last_continuous_improvement_report: Option<ContinuousImprovementReport>,
}
