use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct SnapshotHash([u8; 32]);

impl SnapshotHash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        let mut data = [0u8; 32];
        data.copy_from_slice(&digest);
        Self(data)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}
