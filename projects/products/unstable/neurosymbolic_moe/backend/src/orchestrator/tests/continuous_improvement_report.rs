use crate::dataset_engine::DatasetQualityReport;
use crate::evaluation_engine::EvaluationGovernanceReport;
use crate::orchestrator::ContinuousImprovementReport;

#[test]
fn checksum_fingerprint_is_stable_for_same_inputs() {
    let report = ContinuousImprovementReport {
        governance: EvaluationGovernanceReport {
            min_expert_success_rate: 0.8,
            min_routing_accuracy: 0.9,
            underperforming_experts: vec![],
            routing_accuracy_below_threshold: false,
            ready_for_promotion: true,
        },
        dataset_quality: DatasetQualityReport {
            total_entries: 10,
            scored_entries: 10,
            average_score: Some(0.9),
            low_score_entries: 0,
            corrected_entries: 1,
            correction_ratio: 0.1,
            success_ratio: 0.9,
        },
        expert_regressions: vec![],
        routing_regression: None,
        requires_human_review: false,
    };

    let first = report.checksum_fingerprint();
    let second = report.checksum_fingerprint();
    assert_eq!(first, second);
}
