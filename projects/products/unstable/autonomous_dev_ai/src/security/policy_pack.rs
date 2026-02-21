// projects/products/unstable/autonomous_dev_ai/src/security/policy_pack.rs
use serde::{Deserialize, Serialize};

/// A versioned, signable policy pack loaded at runtime.
///
/// The `forbidden_patterns` list uses case-insensitive substring matching as a
/// first-pass guard. For production hardening, pattern entries should be kept
/// canonical (single-space, lowercase) and supplemented by the `PolicyEngine`'s
/// structural command parser for robust bypass prevention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyPack {
    pub version: String,
    pub forbidden_patterns: Vec<String>,
    pub allowed_tools: Vec<String>,
    /// Optional SHA-256 hex digest of the serialized forbidden_patterns + allowed_tools.
    pub signature: Option<String>,
}

impl PolicyPack {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            forbidden_patterns: vec![
                "rm -rf".to_string(),
                "/etc/".to_string(),
                "sudo ".to_string(),
            ],
            allowed_tools: vec![
                "read_file".to_string(),
                "search_code".to_string(),
                "apply_patch".to_string(),
                "run_tests".to_string(),
                "format_code".to_string(),
                "git_commit".to_string(),
                "git_branch".to_string(),
                "create_pr".to_string(),
                "generate_pr_description".to_string(),
            ],
            signature: None,
        }
    }

    /// Compute a deterministic content fingerprint used to detect accidental tampering.
    ///
    /// Uses FNV-1a 64-bit (stable, deterministic, pure arithmetic) for content
    /// integrity verification only - this is **not** a cryptographic signature.
    pub fn fingerprint(&self) -> String {
        const OFFSET_BASIS: u64 = 14695981039346656037;
        const PRIME: u64 = 1099511628211;

        let mut hash: u64 = OFFSET_BASIS;
        let feed = |hash: &mut u64, bytes: &[u8]| {
            for &b in bytes {
                *hash ^= b as u64;
                *hash = hash.wrapping_mul(PRIME);
            }
        };

        feed(&mut hash, self.version.as_bytes());
        for pattern in &self.forbidden_patterns {
            feed(&mut hash, pattern.as_bytes());
        }
        for tool in &self.allowed_tools {
            feed(&mut hash, tool.as_bytes());
        }
        format!("{hash:016x}")
    }

    /// Sign by storing the fingerprint into `self.signature`.
    pub fn sign(&mut self) {
        self.signature = Some(self.fingerprint());
    }

    /// Verify that the stored signature matches the current content.
    pub fn verify(&self) -> bool {
        match &self.signature {
            None => false,
            Some(sig) => *sig == self.fingerprint(),
        }
    }
}

impl Default for PolicyPack {
    fn default() -> Self {
        let mut pack = Self::new("1.0.0");
        pack.sign();
        pack
    }
}
