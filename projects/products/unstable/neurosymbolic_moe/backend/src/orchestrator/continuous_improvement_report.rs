use crate::dataset_engine::DatasetQualityReport;
use crate::evaluation_engine::{EvaluationGovernanceReport, ExpertRegression, RoutingRegression};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousImprovementReport {
    pub governance: EvaluationGovernanceReport,
    pub dataset_quality: DatasetQualityReport,
    pub expert_regressions: Vec<ExpertRegression>,
    pub routing_regression: Option<RoutingRegression>,
    pub requires_human_review: bool,
}
