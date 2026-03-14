//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/concurrent_operational_report.rs
use crate::orchestrator::{
    ConcurrentLockMetrics, ConcurrentOperationalReport, ImportTelemetry, OperationalReport,
};

#[test]
fn concurrent_operational_report_slo_and_prometheus_helpers_work() {
    let report = ConcurrentOperationalReport {
        pipeline: OperationalReport {
            governance_current_version: 1,
            governance_current_checksum: Some("abcd".to_string()),
            governance_audit_entries: 1,
            governance_state_snapshots: 1,
            runtime_bundle_checksum: "1234".to_string(),
            short_term_memory_entries: 1,
            long_term_memory_entries: 1,
            working_buffer_entries: 0,
            session_buffer_sessions: 0,
            session_buffer_values: 0,
            trace_entries: 1,
            dataset_entries: 0,
            feedback_entries: 0,
            import_telemetry: ImportTelemetry {
                runtime_bundle_import_successes: 1,
                ..ImportTelemetry::default()
            },
            import_journal_events_total: 1,
            import_journal_parse_failures_total: 0,
            import_journal_rejections_total: 0,
            import_journal_successful_imports_total: 1,
            import_journal_deduplicated_replays_total: 0,
            import_journal_tracked_fingerprints: 1,
            runtime_last_import_at_epoch_seconds: None,
            runtime_last_import_released_expired_leases: 0,
            runtime_last_import_observed_dead_letter_events: 0,
            runtime_last_import_pending_events_after_import: 0,
            runtime_last_import_leased_events_after_import: 0,
            runtime_last_import_dead_letter_events_after_import: 0,
            auto_improvement_runs_total: 0,
            auto_improvement_bootstrap_entries_total: 0,
            auto_improvement_last_included_entries: 0,
            auto_improvement_skipped_min_dataset_entries_total: 0,
            auto_improvement_skipped_min_success_ratio_total: 0,
            auto_improvement_skipped_min_average_score_total: 0,
            auto_improvement_skipped_human_review_required_total: 0,
            auto_improvement_skipped_duplicate_bundle_total: 0,
            auto_improvement_build_failures_total: 0,
            model_registry_entries: 0,
            model_registry_active_version: 0,
            model_registry_latest_version: 0,
            trainer_trigger_events_pending: 0,
            trainer_trigger_events_leased: 0,
            trainer_trigger_events_dead_letter: 0,
            trainer_trigger_max_delivery_attempts_pending: 0,
            trainer_trigger_oldest_generated_at_pending: None,
            trainer_trigger_newest_generated_at_pending: None,
            trainer_trigger_oldest_generated_at_dead_letter: None,
            trainer_trigger_newest_generated_at_dead_letter: None,
            trainer_trigger_delivery_attempts_total: 0,
            trainer_trigger_delivery_failures_total: 0,
            trainer_trigger_acknowledged_total: 0,
            trainer_trigger_dead_letter_total: 0,
        },
        lock_metrics: ConcurrentLockMetrics::default(),
        lock_contention_rate: 0.0,
        lock_timeout_rate: 0.0,
        write_guard_rejections: 0,
    };
    assert_eq!(report.slo_status(0.5, 0.1, 1, 0, 0), "OK");
    assert!(
        report
            .to_prometheus_text("moe_concurrent")
            .contains("moe_concurrent_write_guard_rejections")
    );
}
