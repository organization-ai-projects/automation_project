use sha2::{Digest, Sha256};

pub struct RunHash(pub String);

impl RunHash {
    pub fn compute(report_json: &str) -> RunHash {
        let mut hasher = Sha256::new();
        hasher.update(report_json.as_bytes());
        let result = hasher.finalize();
        RunHash(hex::encode(result))
    }
}
