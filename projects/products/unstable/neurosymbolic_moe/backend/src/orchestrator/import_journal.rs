//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/import_journal.rs
use crate::fingerprint::FingerprintManager;

#[derive(Debug, Clone)]
pub struct ImportJournal {
    events_total: u64,
    parse_failures_total: u64,
    rejections_total: u64,
    successful_imports_total: u64,
    deduplicated_replays_total: u64,
    fingerprints: FingerprintManager,
}

impl ImportJournal {
    pub fn with_capacity(fingerprint_capacity: usize) -> Self {
        Self {
            fingerprints: FingerprintManager::new(fingerprint_capacity),
            events_total: 0,
            parse_failures_total: 0,
            rejections_total: 0,
            successful_imports_total: 0,
            deduplicated_replays_total: 0,
        }
    }

    pub fn has_successful_payload_fingerprint(&self, fingerprint: &str) -> bool {
        self.fingerprints.contains(fingerprint)
    }

    pub fn record_successful_import(&mut self, payload_fingerprint: String) {
        self.fingerprints.add_fingerprint(payload_fingerprint);
        self.successful_imports_total += 1;
        self.events_total += 1;
    }

    pub fn record_rejection(&mut self) {
        self.rejections_total += 1;
        self.events_total += 1;
    }

    pub fn record_parse_failure(&mut self) {
        self.parse_failures_total += 1;
        self.events_total += 1;
    }

    pub fn record_deduplicated_replay(&mut self) {
        self.deduplicated_replays_total += 1;
        self.events_total += 1;
    }

    pub fn events_total(&self) -> u64 {
        self.events_total
    }

    pub fn parse_failures_total(&self) -> u64 {
        self.parse_failures_total
    }

    pub fn rejections_total(&self) -> u64 {
        self.rejections_total
    }

    pub fn successful_imports_total(&self) -> u64 {
        self.successful_imports_total
    }

    pub fn deduplicated_replays_total(&self) -> u64 {
        self.deduplicated_replays_total
    }

    pub fn tracked_fingerprint_count(&self) -> usize {
        self.fingerprints.count()
    }
}

impl Default for ImportJournal {
    fn default() -> Self {
        Self {
            events_total: 0,
            parse_failures_total: 0,
            rejections_total: 0,
            successful_imports_total: 0,
            deduplicated_replays_total: 0,
            fingerprints: FingerprintManager::new(256),
        }
    }
}
