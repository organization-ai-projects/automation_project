use crate::diagnostics::SpaceEmpireError;
use crate::io::JsonCodec;
use crate::replay::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(replay: &ReplayFile) -> Result<Vec<u8>, SpaceEmpireError> {
        let s = JsonCodec::encode(replay)?;
        Ok(s.into_bytes())
    }

    pub fn decode(bytes: &[u8]) -> Result<ReplayFile, SpaceEmpireError> {
        let s = std::str::from_utf8(bytes)
            .map_err(|e| SpaceEmpireError::Serialization(e.to_string()))?;
        JsonCodec::decode(s)
    }
}
