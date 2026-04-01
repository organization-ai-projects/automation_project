use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub entries: Vec<ManifestEntry>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub path: String,
    pub size: usize,
    pub sha256: String,
}

pub fn compute_manifest_hash(artifacts: &[(String, Vec<u8>)]) -> String {
    let mut hasher = Sha256::new();
    let mut sorted: Vec<_> = artifacts.iter().collect();
    sorted.sort_by_key(|(path, _)| path.as_str());
    for (path, content) in sorted {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
        hasher.update(content);
        hasher.update(b"\0");
    }
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        let artifacts = vec![
            ("a.rs".to_string(), b"hello".to_vec()),
            ("b.rs".to_string(), b"world".to_vec()),
        ];
        let h1 = compute_manifest_hash(&artifacts);
        let h2 = compute_manifest_hash(&artifacts);
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }

    #[test]
    fn hash_is_order_independent() {
        let a = vec![
            ("a.rs".to_string(), b"hello".to_vec()),
            ("b.rs".to_string(), b"world".to_vec()),
        ];
        let b = vec![
            ("b.rs".to_string(), b"world".to_vec()),
            ("a.rs".to_string(), b"hello".to_vec()),
        ];
        assert_eq!(compute_manifest_hash(&a), compute_manifest_hash(&b));
    }
}
