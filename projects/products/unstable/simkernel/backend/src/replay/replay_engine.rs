#![allow(dead_code)]
use crate::replay::replay_file::ReplayFile;
use crate::diagnostics::error::SimError;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(file: &ReplayFile) -> Result<u64, SimError> {
        if file.pack_id.is_empty() {
            return Err(SimError::ReplayMismatch("empty pack_id".to_string()));
        }
        Ok(file.event_log_checksum)
    }
}
