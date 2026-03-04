use crate::diagnostics::backend_error::BackendError;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(file: &ReplayFile) -> Result<u64, BackendError> {
        if file.pack_id.is_empty() {
            return Err(BackendError::ReplayMismatch("empty pack_id".to_string()));
        }
        Ok(file.event_log_checksum)
    }
}
