use crate::replay::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(replay: &ReplayFile) -> Result<String, String> {
        common_json::to_json_string_pretty(replay).map_err(|e| format!("replay encode failed: {e}"))
    }

    pub fn decode(json: &str) -> Result<ReplayFile, String> {
        common_json::from_str(json).map_err(|e| format!("replay decode failed: {e}"))
    }
}
