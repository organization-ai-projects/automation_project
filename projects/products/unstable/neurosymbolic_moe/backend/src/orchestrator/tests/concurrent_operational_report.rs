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
            auto_improvement_runs_total: 0,
            auto_improvement_bootstrap_entries_total: 0,
            auto_improvement_last_included_entries: 0,
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
