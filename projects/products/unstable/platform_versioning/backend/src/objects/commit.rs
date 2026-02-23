// projects/products/unstable/platform_versioning/backend/src/objects/commit.rs
use serde::{Deserialize, Serialize};

use crate::ids::{CommitId, TreeId};
use crate::objects::HashDigest;

/// An immutable, content-addressed commit record.
///
/// # Encoding format (version 1)
/// The canonical byte sequence fed to SHA-256 is:
/// ```text
/// b"commit\x00" ++ version(u8) ++ tree_id(64 hex bytes) ++
/// parent_count(u32 LE) ++ (parent_id(64 hex bytes))* ++
/// author_len(u16 LE) ++ author_bytes ++
/// message_len(u32 LE) ++ message_bytes ++
/// timestamp_secs(u64 LE)
/// ```
/// Determinism rule: given identical inputs the output commit id is identical.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commit {
    /// The content address of this commit.
    pub id: CommitId,
    /// The root tree snapshot for this commit.
    pub tree_id: TreeId,
    /// Parent commit ids, ordered (typically 0 or 1; 2 for merges).
    pub parent_ids: Vec<CommitId>,
    /// Author identifier (display name or email — opaque to the core).
    pub author: String,
    /// Human-readable commit message.
    pub message: String,
    /// Unix timestamp (seconds since epoch) when this commit was created.
    pub timestamp_secs: u64,
}

const COMMIT_FORMAT_VERSION: u8 = 1;

impl Commit {
    /// Computes the deterministic [`CommitId`] for the given fields.
    pub fn compute_id(
        tree_id: &TreeId,
        parent_ids: &[CommitId],
        author: &str,
        message: &str,
        timestamp_secs: u64,
    ) -> CommitId {
        let mut parts: Vec<Vec<u8>> = Vec::new();
        parts.push(b"commit\x00".to_vec());
        parts.push(vec![COMMIT_FORMAT_VERSION]);
        parts.push(tree_id.as_str().as_bytes().to_vec());
        let parent_count = (parent_ids.len() as u32).to_le_bytes();
        parts.push(parent_count.to_vec());
        for pid in parent_ids {
            parts.push(pid.as_str().as_bytes().to_vec());
        }
        let author_bytes = author.as_bytes();
        let author_len = (author_bytes.len() as u16).to_le_bytes();
        parts.push(author_len.to_vec());
        parts.push(author_bytes.to_vec());
        let msg_bytes = message.as_bytes();
        let msg_len = (msg_bytes.len() as u32).to_le_bytes();
        parts.push(msg_len.to_vec());
        parts.push(msg_bytes.to_vec());
        parts.push(timestamp_secs.to_le_bytes().to_vec());

        let refs: Vec<&[u8]> = parts.iter().map(|v| v.as_slice()).collect();
        let digest = HashDigest::compute_parts(&refs);
        CommitId::from_bytes(&digest)
    }

    /// Creates a new `Commit`, computing the content address from the supplied fields.
    pub fn new(
        tree_id: TreeId,
        parent_ids: Vec<CommitId>,
        author: String,
        message: String,
        timestamp_secs: u64,
    ) -> Self {
        let id =
            Self::compute_id(&tree_id, &parent_ids, &author, &message, timestamp_secs);
        Self {
            id,
            tree_id,
            parent_ids,
            author,
            message,
            timestamp_secs,
        }
    }

    /// Validates that the stored `id` matches the recomputed address of the fields.
    pub fn verify(&self) -> bool {
        Self::compute_id(
            &self.tree_id,
            &self.parent_ids,
            &self.author,
            &self.message,
            self.timestamp_secs,
        ) == self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ObjectId;

    fn dummy_tree_id() -> TreeId {
        TreeId::from_bytes(&[0xaau8; 32])
    }

    #[test]
    fn id_is_deterministic() {
        let tree = dummy_tree_id();
        let a = Commit::new(
            tree.clone(),
            vec![],
            "Alice".to_string(),
            "Initial commit".to_string(),
            1_700_000_000,
        );
        let b = Commit::new(
            tree,
            vec![],
            "Alice".to_string(),
            "Initial commit".to_string(),
            1_700_000_000,
        );
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn different_message_different_id() {
        let tree = dummy_tree_id();
        let a = Commit::new(
            tree.clone(),
            vec![],
            "Alice".to_string(),
            "First".to_string(),
            0,
        );
        let b = Commit::new(
            tree,
            vec![],
            "Alice".to_string(),
            "Second".to_string(),
            0,
        );
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn verify_intact() {
        let commit = Commit::new(
            dummy_tree_id(),
            vec![],
            "Bob".to_string(),
            "msg".to_string(),
            42,
        );
        assert!(commit.verify());
    }

    #[test]
    fn verify_corrupt() {
        let mut commit = Commit::new(
            dummy_tree_id(),
            vec![],
            "Bob".to_string(),
            "msg".to_string(),
            42,
        );
        commit.message = "tampered".to_string();
        assert!(!commit.verify());
    }

    #[test]
    fn commit_with_parents() {
        let tree = dummy_tree_id();
        let parent_raw = [0x01u8; 32];
        let parent_id = CommitId::from(ObjectId::from_bytes(&parent_raw));
        let commit = Commit::new(
            tree,
            vec![parent_id],
            "Carol".to_string(),
            "second".to_string(),
            100,
        );
        assert!(commit.verify());
        assert_eq!(commit.parent_ids.len(), 1);
    }
}
