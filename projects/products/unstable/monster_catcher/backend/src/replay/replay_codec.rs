use crate::diagnostics::error::BackendError;
use crate::io::canonical_json::to_canonical_string;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(file: &ReplayFile) -> Result<String, BackendError> {
        to_canonical_string(file).map_err(BackendError::Codec)
    }

    pub fn decode(raw: &str) -> Result<ReplayFile, BackendError> {
        common_json::from_json_str(raw).map_err(|e| BackendError::Replay(e.to_string()))
    }
}
