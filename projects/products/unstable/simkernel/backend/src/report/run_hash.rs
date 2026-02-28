#![allow(dead_code)]
use crate::events::event_log::EventLog;
use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(event_log: &EventLog, snapshot_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(event_log.checksum().to_le_bytes());
        hasher.update(snapshot_hash.as_bytes());
        hex::encode(hasher.finalize())
    }
}
