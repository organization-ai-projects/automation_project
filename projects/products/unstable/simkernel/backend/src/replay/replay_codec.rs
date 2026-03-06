use crate::diagnostics::backend_error::BackendError;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(file: &ReplayFile) -> Result<String, BackendError> {
        common_json::to_string(file).map_err(|e| BackendError::Serialization(e.to_string()))
    }

    pub fn decode(data: &str) -> Result<ReplayFile, BackendError> {
        common_json::from_str(data).map_err(|e| BackendError::Serialization(e.to_string()))
    }
}
