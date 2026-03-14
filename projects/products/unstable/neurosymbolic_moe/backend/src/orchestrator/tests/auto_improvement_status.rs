//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/auto_improvement_status.rs
use crate::orchestrator::AutoImprovementStatus;

#[test]
fn auto_improvement_status_default_is_empty() {
    let status = AutoImprovementStatus::default();
    assert_eq!(status.runs_total, 0);
    assert_eq!(status.bootstrap_entries_total, 0);
    assert!(status.last_bundle_checksum.is_none());
    assert_eq!(status.last_included_entries, 0);
    assert_eq!(status.last_train_samples, 0);
    assert_eq!(status.last_validation_samples, 0);
    assert_eq!(status.skipped_min_dataset_entries_total, 0);
    assert_eq!(status.skipped_min_success_ratio_total, 0);
    assert_eq!(status.skipped_min_average_score_total, 0);
    assert_eq!(status.skipped_human_review_required_total, 0);
    assert_eq!(status.skipped_duplicate_bundle_total, 0);
    assert_eq!(status.build_failures_total, 0);
    assert!(status.last_skip_reason.is_none());
    assert_eq!(status.trainer_trigger_delivery_attempts_total, 0);
    assert_eq!(status.trainer_trigger_delivery_failures_total, 0);
    assert_eq!(status.trainer_trigger_acknowledged_total, 0);
}

#[test]
fn auto_improvement_status_fields_roundtrip() {
    let status = AutoImprovementStatus {
        runs_total: 3,
        bootstrap_entries_total: 12,
        last_bundle_checksum: Some("abc123".to_string()),
        last_included_entries: 24,
        last_train_samples: 20,
        last_validation_samples: 4,
        skipped_min_dataset_entries_total: 1,
        skipped_min_success_ratio_total: 2,
        skipped_min_average_score_total: 3,
        skipped_human_review_required_total: 4,
        skipped_duplicate_bundle_total: 5,
        build_failures_total: 6,
        last_skip_reason: Some("test skip reason".to_string()),
        trainer_trigger_delivery_attempts_total: 7,
        trainer_trigger_delivery_failures_total: 8,
        trainer_trigger_acknowledged_total: 9,
    };
    assert_eq!(status.runs_total, 3);
    assert_eq!(status.bootstrap_entries_total, 12);
    assert_eq!(status.last_bundle_checksum.as_deref(), Some("abc123"));
    assert_eq!(status.last_included_entries, 24);
    assert_eq!(status.last_train_samples, 20);
    assert_eq!(status.last_validation_samples, 4);
    assert_eq!(status.skipped_min_dataset_entries_total, 1);
    assert_eq!(status.skipped_min_success_ratio_total, 2);
    assert_eq!(status.skipped_min_average_score_total, 3);
    assert_eq!(status.skipped_human_review_required_total, 4);
    assert_eq!(status.skipped_duplicate_bundle_total, 5);
    assert_eq!(status.build_failures_total, 6);
    assert_eq!(status.last_skip_reason.as_deref(), Some("test skip reason"));
    assert_eq!(status.trainer_trigger_delivery_attempts_total, 7);
    assert_eq!(status.trainer_trigger_delivery_failures_total, 8);
    assert_eq!(status.trainer_trigger_acknowledged_total, 9);
}
