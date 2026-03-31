use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThreatId(pub String);

impl ThreatId {
    pub fn new() -> Self {
        use sha2::{Digest, Sha256};
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let mut hasher = Sha256::new();
        hasher.update(timestamp.to_le_bytes());
        let hash = hasher.finalize();
        Self(hex::encode(&hash[..16]))
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for ThreatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
