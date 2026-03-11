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

impl ContinuousImprovementReport {
    pub fn checksum_fingerprint(&self) -> String {
        let mut regressions = self
            .expert_regressions
            .iter()
            .map(|r| {
                format!(
                    "{}:{:.6}:{:.6}:{:.6}",
                    r.expert_id.as_str(),
                    r.previous_success_rate,
                    r.current_success_rate,
                    r.delta
                )
            })
            .collect::<Vec<_>>();
        regressions.sort();

        let routing = self
            .routing_regression
            .as_ref()
            .map(|r| {
                format!(
                    "{:.6}:{:.6}:{:.6}",
                    r.previous_accuracy, r.current_accuracy, r.delta
                )
            })
            .unwrap_or_else(|| "-".to_string());

        let mut underperforming = self
            .governance
            .underperforming_experts
            .iter()
            .map(|id| id.as_str().to_string())
            .collect::<Vec<_>>();
        underperforming.sort();

        let governance = format!(
            "{:.6}:{:.6}:{}:{}:{}",
            self.governance.min_expert_success_rate,
            self.governance.min_routing_accuracy,
            underperforming.join(","),
            self.governance.routing_accuracy_below_threshold,
            self.governance.ready_for_promotion
        );

        let dataset = format!(
            "{}:{}:{:.6}:{}:{}:{:.6}:{:.6}",
            self.dataset_quality.total_entries,
            self.dataset_quality.scored_entries,
            self.dataset_quality.average_score.unwrap_or(-1.0),
            self.dataset_quality.low_score_entries,
            self.dataset_quality.corrected_entries,
            self.dataset_quality.correction_ratio,
            self.dataset_quality.success_ratio
        );

        format!(
            "gov={};dataset={};regr=[{}];routing={};review={}",
            governance,
            dataset,
            regressions.join("|"),
            routing,
            self.requires_human_review
        )
    }
}
