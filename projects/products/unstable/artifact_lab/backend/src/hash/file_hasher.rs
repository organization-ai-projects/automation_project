use sha2::{Digest, Sha256};

pub struct FileHasher;

impl FileHasher {
    pub fn hash_bytes(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}
