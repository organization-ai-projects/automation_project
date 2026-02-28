use crate::diagnostics::error::SpaceDiploWarsError;
use crate::io::json_codec::JsonCodec;

use super::replay_file::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(replay: &ReplayFile) -> Result<String, SpaceDiploWarsError> {
        JsonCodec::encode(replay)
    }

    pub fn decode(json: &str) -> Result<ReplayFile, SpaceDiploWarsError> {
        JsonCodec::decode(json)
    }
}
