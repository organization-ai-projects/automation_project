#![allow(dead_code)]
use crate::events::event_log::EventLog;
use sha2::{Digest, Sha256};

/// Derives the canonical RunHash from the event log checksum and scenario hash.
pub struct RunHash;

impl RunHash {
    pub fn compute(event_checksum: u64, scenario_hash: &str, ticks: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(event_checksum.to_le_bytes());
        hasher.update(scenario_hash.as_bytes());
        hasher.update(ticks.to_le_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn from_report_json(canonical_json: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(canonical_json.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn from_event_log(event_log: &EventLog, scenario_hash: &str, ticks: u64) -> String {
        Self::compute(event_log.checksum(), scenario_hash, ticks)
    }
}
