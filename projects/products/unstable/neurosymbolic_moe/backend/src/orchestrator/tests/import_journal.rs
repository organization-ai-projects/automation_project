//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/import_journal.rs
use crate::orchestrator::import_journal::ImportJournal;

#[test]
fn import_journal_tracks_success_rejection_parse_failure_and_dedup() {
    let mut journal = ImportJournal::with_capacity(2);

    // Record successful imports
    journal.record_successful_import("item1".to_string());
    journal.record_successful_import("item2".to_string());
    assert!(journal.has_successful_payload_fingerprint("item1"));
    assert!(journal.has_successful_payload_fingerprint("item2"));

    // Add a new import and verify the oldest is removed
    journal.record_successful_import("item3".to_string());
    assert!(!journal.has_successful_payload_fingerprint("item1"));
    assert!(journal.has_successful_payload_fingerprint("item3"));

    // Record additional events
    journal.record_parse_failure();
    journal.record_rejection();
    journal.record_deduplicated_replay();

    // Verify totals
    assert_eq!(journal.parse_failures_total(), 1);
    assert_eq!(journal.rejections_total(), 1);
    assert_eq!(journal.deduplicated_replays_total(), 1);
    assert_eq!(journal.successful_imports_total(), 3);
    assert_eq!(journal.tracked_fingerprint_count(), 2);
    assert_eq!(journal.events_total(), 6);
}
