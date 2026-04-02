//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/auto_improvement_status.rs
use crate::{
    delivery_stats::DeliveryStats, global_counters::GlobalCounters,
    orchestrator::AutoImprovementStatus, skip_counters::SkipCounters,
};

#[test]
fn auto_improvement_status_default_is_empty() {
    let status = AutoImprovementStatus::default();
    assert_eq!(status.global_counters.runs_total, 0);
    assert_eq!(status.global_counters.bootstrap_entries_total, 0);
    assert!(status.last_bundle_checksum.is_none());
    assert_eq!(status.last_included_entries, 0);
    assert_eq!(status.last_train_samples, 0);
    assert_eq!(status.last_validation_samples, 0);
    assert_eq!(status.skip_counters.min_dataset_entries_total, 0);
    assert_eq!(status.skip_counters.min_success_ratio_total, 0);
    assert_eq!(status.skip_counters.min_average_score_total, 0);
    assert_eq!(status.skip_counters.human_review_required_total, 0);
    assert_eq!(status.skip_counters.duplicate_bundle_total, 0);
    assert_eq!(status.global_counters.build_failures_total, 0);
    assert!(status.last_skip_reason.is_none());
    assert_eq!(status.delivery_stats.delivery_attempts_total, 0);
    assert_eq!(status.delivery_stats.delivery_failures_total, 0);
    assert_eq!(status.delivery_stats.acknowledged_total, 0);
    assert_eq!(status.delivery_stats.dead_letter_total, 0);
}

#[test]
fn auto_improvement_status_fields_roundtrip() {
    let status = AutoImprovementStatus {
        global_counters: GlobalCounters {
            runs_total: 3,
            bootstrap_entries_total: 12,
            build_failures_total: 6,
        },
        skip_counters: SkipCounters {
            min_dataset_entries_total: 1,
            min_success_ratio_total: 2,
            min_average_score_total: 3,
            human_review_required_total: 4,
            duplicate_bundle_total: 5,
        },
        delivery_stats: DeliveryStats {
            delivery_attempts_total: 7,
            delivery_failures_total: 8,
            acknowledged_total: 9,
            dead_letter_total: 10,
        },
        last_bundle_checksum: Some("abc123".to_string()),
        last_included_entries: 24,
        last_train_samples: 20,
        last_validation_samples: 4,
        last_skip_reason: Some("test skip reason".to_string()),
    };
    assert_eq!(status.global_counters.runs_total, 3);
    assert_eq!(status.global_counters.bootstrap_entries_total, 12);
    assert_eq!(status.last_bundle_checksum.as_deref(), Some("abc123"));
    assert_eq!(status.last_included_entries, 24);
    assert_eq!(status.last_train_samples, 20);
    assert_eq!(status.last_validation_samples, 4);
    assert_eq!(status.skip_counters.min_dataset_entries_total, 1);
    assert_eq!(status.skip_counters.min_success_ratio_total, 2);
    assert_eq!(status.skip_counters.min_average_score_total, 3);
    assert_eq!(status.skip_counters.human_review_required_total, 4);
    assert_eq!(status.skip_counters.duplicate_bundle_total, 5);
    assert_eq!(status.global_counters.build_failures_total, 6);
    assert_eq!(status.last_skip_reason.as_deref(), Some("test skip reason"));
    assert_eq!(status.delivery_stats.delivery_attempts_total, 7);
    assert_eq!(status.delivery_stats.delivery_failures_total, 8);
    assert_eq!(status.delivery_stats.acknowledged_total, 9);
    assert_eq!(status.delivery_stats.dead_letter_total, 10);
}
