use crate::diagnostics::Error;
use crate::replay::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(replay: &ReplayFile) -> Result<String, Error> {
        common_json::to_string_pretty(replay).map_err(|e| Error::Serialization(e.to_string()))
    }

    pub fn decode(data: &str) -> Result<ReplayFile, Error> {
        common_json::from_str(data).map_err(|e| Error::Deserialization(e.to_string()))
    }
}
