// projects/products/unstable/platform_versioning/backend/src/objects/tree.rs
use serde::{Deserialize, Serialize};

use crate::ids::TreeId;
use crate::objects::{HashDigest, TreeEntry, TreeEntryKind};

/// An immutable content-addressed directory snapshot.
///
/// # Encoding format (version 1)
/// Entries are sorted lexicographically by `name` before encoding.
/// Each entry contributes:
/// ```text
/// kind_byte(1) ++ name_len(u16 LE) ++ name_bytes ++ id_hex(64)
/// ```
/// The full encoding is prefixed by `b"tree\x00"` and a `u32 LE` entry count.
/// The SHA-256 of the above is the canonical [`TreeId`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tree {
    /// The content address of this tree.
    pub id: TreeId,
    /// The entries in this tree, sorted lexicographically by name.
    pub entries: Vec<TreeEntry>,
}

const TREE_FORMAT_VERSION: u8 = 1;

impl Tree {
    /// Creates a `Tree` from unsorted entries, sorting and computing the id.
    pub fn from_entries(mut entries: Vec<TreeEntry>) -> Self {
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        let id = Self::compute_id(&entries);
        Self { id, entries }
    }

    /// Computes the [`TreeId`] for the given (already-sorted) entries.
    pub fn compute_id(entries: &[TreeEntry]) -> TreeId {
        let mut parts: Vec<Vec<u8>> = Vec::new();
        parts.push(vec![TREE_FORMAT_VERSION]);
        parts.push(b"tree\x00".to_vec());
        let count = (entries.len() as u32).to_le_bytes();
        parts.push(count.to_vec());
        for entry in entries {
            let kind_byte: u8 = match entry.kind {
                TreeEntryKind::Blob => 0,
                TreeEntryKind::Tree => 1,
            };
            parts.push(vec![kind_byte]);
            let name_bytes = entry.name.as_bytes();
            let name_len = (name_bytes.len() as u16).to_le_bytes();
            parts.push(name_len.to_vec());
            parts.push(name_bytes.to_vec());
            parts.push(entry.id.as_str().as_bytes().to_vec());
        }
        let refs: Vec<&[u8]> = parts.iter().map(|v| v.as_slice()).collect();
        let digest = HashDigest::compute_parts(&refs);
        TreeId::from_bytes(&digest)
    }

    /// Validates that the stored `id` matches the recomputed address of `entries`.
    pub fn verify(&self) -> bool {
        // Entries must be sorted for the invariant to hold.
        let sorted = self.entries.windows(2).all(|w| w[0].name <= w[1].name);
        sorted && Self::compute_id(&self.entries) == self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ObjectId;

    fn dummy_oid(byte: u8) -> ObjectId {
        ObjectId::from_bytes(&[byte; 32])
    }

    #[test]
    fn empty_tree_is_deterministic() {
        let a = Tree::from_entries(vec![]);
        let b = Tree::from_entries(vec![]);
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn entries_are_sorted() {
        let e1 = TreeEntry {
            name: "z_file.txt".to_string(),
            kind: TreeEntryKind::Blob,
            id: dummy_oid(1),
        };
        let e2 = TreeEntry {
            name: "a_file.txt".to_string(),
            kind: TreeEntryKind::Blob,
            id: dummy_oid(2),
        };
        let tree = Tree::from_entries(vec![e1, e2]);
        assert_eq!(tree.entries[0].name, "a_file.txt");
    }

    #[test]
    fn verify_intact() {
        let tree = Tree::from_entries(vec![]);
        assert!(tree.verify());
    }

    #[test]
    fn verify_corrupt() {
        let mut tree = Tree::from_entries(vec![]);
        tree.entries.push(TreeEntry {
            name: "injected".to_string(),
            kind: TreeEntryKind::Blob,
            id: dummy_oid(0),
        });
        assert!(!tree.verify());
    }
}
