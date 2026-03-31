use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(pub String);

impl MessageId {
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
}
