use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ImportJournal {
    events_total: u64,
    parse_failures_total: u64,
    rejections_total: u64,
    successful_imports_total: u64,
    deduplicated_replays_total: u64,
    recent_successful_payload_fingerprints: VecDeque<String>,
    fingerprint_capacity: usize,
}

impl ImportJournal {
    pub fn with_capacity(fingerprint_capacity: usize) -> Self {
        Self {
            fingerprint_capacity: fingerprint_capacity.max(1),
            ..Self::default()
        }
    }

    pub fn payload_fingerprint(payload: &str) -> String {
        format!("{:016x}", fnv1a64(payload.as_bytes()))
    }

    pub fn has_successful_payload_fingerprint(&self, fingerprint: &str) -> bool {
        self.recent_successful_payload_fingerprints
            .iter()
            .any(|stored| stored == fingerprint)
    }

    pub fn record_successful_import(&mut self, payload_fingerprint: String) {
        self.events_total += 1;
        self.successful_imports_total += 1;
        self.recent_successful_payload_fingerprints
            .retain(|existing| existing != &payload_fingerprint);
        self.recent_successful_payload_fingerprints
            .push_back(payload_fingerprint);
        while self.recent_successful_payload_fingerprints.len() > self.fingerprint_capacity {
            self.recent_successful_payload_fingerprints.pop_front();
        }
    }

    pub fn record_rejection(&mut self) {
        self.events_total += 1;
        self.rejections_total += 1;
    }

    pub fn record_parse_failure(&mut self) {
        self.events_total += 1;
        self.parse_failures_total += 1;
    }

    pub fn record_deduplicated_replay(&mut self) {
        self.events_total += 1;
        self.deduplicated_replays_total += 1;
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
        self.recent_successful_payload_fingerprints.len()
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
            recent_successful_payload_fingerprints: VecDeque::new(),
            fingerprint_capacity: 256,
        }
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
