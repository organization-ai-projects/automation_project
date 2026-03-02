use crate::diagnostics::LifeSimError;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    #[allow(dead_code)]
    pub fn serialize(file: &ReplayFile) -> Result<String, LifeSimError> {
        serde_json::to_string_pretty(file).map_err(|e| LifeSimError::Serialization(e.to_string()))
    }

    pub fn deserialize(data: &str) -> Result<ReplayFile, LifeSimError> {
        serde_json::from_str(data).map_err(|e| LifeSimError::Serialization(e.to_string()))
    }
}
