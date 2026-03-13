//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/import_journal.rs
use crate::orchestrator::import_journal::ImportJournal;

#[test]
fn import_journal_tracks_success_rejection_parse_failure_and_dedup() {
    let mut journal = ImportJournal::with_capacity(2);
    let fp1 = ImportJournal::payload_fingerprint("payload-1");
    let fp2 = ImportJournal::payload_fingerprint("payload-2");
    let fp3 = ImportJournal::payload_fingerprint("payload-3");

    journal.record_successful_import(fp1.clone());
    journal.record_successful_import(fp2.clone());
    assert!(journal.has_successful_payload_fingerprint(&fp1));
    assert!(journal.has_successful_payload_fingerprint(&fp2));

    journal.record_successful_import(fp3.clone());
    assert!(!journal.has_successful_payload_fingerprint(&fp1));
    assert!(journal.has_successful_payload_fingerprint(&fp3));

    journal.record_parse_failure();
    journal.record_rejection();
    journal.record_deduplicated_replay();

    assert_eq!(journal.parse_failures_total(), 1);
    assert_eq!(journal.rejections_total(), 1);
    assert_eq!(journal.deduplicated_replays_total(), 1);
    assert_eq!(journal.successful_imports_total(), 3);
    assert_eq!(journal.tracked_fingerprint_count(), 2);
    assert_eq!(journal.events_total(), 6);
}
