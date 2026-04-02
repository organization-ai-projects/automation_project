use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHash(pub String);

impl RunHash {
    pub fn compute(data: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        RunHash(hex::encode(result))
    }
}
