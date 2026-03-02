// projects/products/unstable/simulation_compiler/backend/src/output/manifest_hash.rs
use sha2::{Digest, Sha256};

/// Compute a deterministic SHA-256 hash over the sorted artifact list.
pub fn compute_hash(artifacts: &[(String, Vec<u8>)]) -> String {
    let mut hasher = Sha256::new();
    // Sort by path to guarantee a stable order regardless of insertion order.
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
        let a = vec![
            ("a.rs".to_string(), b"hello".to_vec()),
            ("b.rs".to_string(), b"world".to_vec()),
        ];
        assert_eq!(compute_hash(&a), compute_hash(&a));
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
        assert_eq!(compute_hash(&a), compute_hash(&b));
    }
}
