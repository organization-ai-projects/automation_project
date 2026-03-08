use common_json::Json;

pub struct CanonicalDecoder;

impl CanonicalDecoder {
    pub fn decode_value(bytes: &[u8]) -> Result<Json, String> {
        common_json::from_slice(bytes).map_err(|e| format!("canonical decode failed: {e}"))
    }
}
