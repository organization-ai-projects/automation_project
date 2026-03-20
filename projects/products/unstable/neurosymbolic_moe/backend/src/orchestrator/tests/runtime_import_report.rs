//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/runtime_import_report.rs
use crate::orchestrator::RuntimeImportReport;

#[test]
fn runtime_import_report_fields_roundtrip() {
    let report = RuntimeImportReport {
        imported_at_epoch_seconds: 123,
        runtime_schema_version: 1,
        released_expired_leases: 2,
        observed_dead_letter_events: 3,
        pending_events_after_import: 4,
        leased_events_after_import: 5,
        dead_letter_events_after_import: 6,
        runtime_checksum_after_import: "abcd1234".to_string(),
    };
    assert_eq!(report.imported_at_epoch_seconds, 123);
    assert_eq!(report.runtime_schema_version, 1);
    assert_eq!(report.released_expired_leases, 2);
    assert_eq!(report.observed_dead_letter_events, 3);
    assert_eq!(report.pending_events_after_import, 4);
    assert_eq!(report.leased_events_after_import, 5);
    assert_eq!(report.dead_letter_events_after_import, 6);
    assert_eq!(report.runtime_checksum_after_import, "abcd1234");
}
