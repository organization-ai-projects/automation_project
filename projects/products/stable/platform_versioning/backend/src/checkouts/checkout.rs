// projects/products/stable/platform_versioning/backend/src/checkout/checkout.rs
use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::checkouts::{CheckoutPolicy, Materialized};
use crate::errors::PvError;
use crate::ids::{CommitId, ObjectId};
use crate::indexes::SafePath;
use crate::objects::{Object, ObjectStore};

/// Materializes a commit revision into a target directory.
///
/// # Safe file operations
/// - All destination paths are constructed by joining `dest` with each
///   [`SafePath`], which guarantees no traversal outside `dest`.
/// - Writes use a temp-file-then-rename pattern for atomicity.
pub struct Checkout;

impl Checkout {
    /// Materializes the revision identified by `commit_id` into `dest`.
    ///
    /// Returns a [`Materialized`] summary of files written and deleted.
    pub fn materialize(
        commit_id: &CommitId,
        object_store: &ObjectStore,
        dest: &Path,
        policy: &CheckoutPolicy,
    ) -> Result<Materialized, PvError> {
        // Resolve commit → root tree.
        let commit_obj = object_store.read(commit_id.as_object_id())?;
        let root_tree_id = match commit_obj {
            Object::Commit(ref c) => c.tree_id.clone(),
            _ => return Err(PvError::Internal(format!("{commit_id} is not a commit"))),
        };

        // Flatten the tree into path → blob_id map.
        let files = flatten_tree(root_tree_id.as_object_id(), "", object_store)?;

        // Compute existing files if delete_untracked is enabled.
        let existing: Vec<SafePath> = if policy.delete_untracked {
            collect_existing_files(dest)
        } else {
            vec![]
        };

        let mut files_written = 0usize;
        let mut files_deleted = 0usize;

        for (safe_path, blob_id) in &files {
            let file_dest = dest.join(safe_path.as_str());

            if file_dest.exists() && !policy.overwrite {
                // Check if content differs.
                if let Ok(existing_bytes) = fs::read(&file_dest) {
                    let blob_obj = object_store.read(blob_id)?;
                    if let Object::Blob(blob) = blob_obj {
                        if existing_bytes != blob.content {
                            return Err(PvError::Internal(format!(
                                "conflict: {} exists and differs from revision",
                                safe_path
                            )));
                        }
                        // Content identical — skip write.
                        continue;
                    }
                }
            }

            // Write the file atomically.
            let blob_obj = object_store.read(blob_id)?;
            if let Object::Blob(blob) = blob_obj {
                atomic_write_file(&file_dest, &blob.content)?;
                files_written += 1;
            }
        }

        // Delete untracked files.
        if policy.delete_untracked {
            let staged_set: std::collections::HashSet<&SafePath> = files.keys().collect();
            for path in &existing {
                if !staged_set.contains(path) {
                    let file_dest = dest.join(path.as_str());
                    if file_dest.exists() {
                        fs::remove_file(&file_dest).map_err(PvError::Io)?;
                        files_deleted += 1;
                    }
                }
            }
        }

        Ok(Materialized {
            commit_id: commit_id.clone(),
            files_written,
            files_deleted,
        })
    }
}

fn flatten_tree(
    tree_id: &ObjectId,
    prefix: &str,
    store: &ObjectStore,
) -> Result<BTreeMap<SafePath, ObjectId>, PvError> {
    let obj = store.read(tree_id)?;
    let tree = match obj {
        Object::Tree(t) => t,
        _ => return Err(PvError::Internal(format!("{tree_id} is not a tree"))),
    };

    let mut result = BTreeMap::new();
    let mut queue: VecDeque<(String, crate::objects::Tree)> = VecDeque::new();
    queue.push_back((prefix.to_string(), tree));

    while let Some((dir, tree)) = queue.pop_front() {
        for entry in tree.entries {
            let path_str = if dir.is_empty() {
                entry.name.clone()
            } else {
                format!("{}/{}", dir, entry.name)
            };
            match entry.kind {
                crate::objects::TreeEntryKind::Blob => {
                    if let Ok(safe) = path_str.parse::<SafePath>() {
                        result.insert(safe, entry.id);
                    }
                }
                crate::objects::TreeEntryKind::Tree => {
                    let sub_obj = store.read(&entry.id)?;
                    if let Object::Tree(sub_tree) = sub_obj {
                        queue.push_back((path_str, sub_tree));
                    }
                }
            }
        }
    }

    Ok(result)
}

fn collect_existing_files(dir: &Path) -> Vec<SafePath> {
    let mut out = Vec::new();
    collect_recursive(dir, dir, &mut out);
    out
}

fn collect_recursive(base: &Path, current: &Path, out: &mut Vec<SafePath>) {
    if let Ok(entries) = fs::read_dir(current) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_recursive(base, &path, out);
            } else if let Ok(rel) = path.strip_prefix(base)
                && let Some(s) = rel.to_str()
                && let Ok(safe) = s.replace('\\', "/").parse::<SafePath>()
            {
                out.push(safe);
            }
        }
    }
}

fn atomic_write_file(path: &Path, data: &[u8]) -> Result<(), PvError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(PvError::Io)?;
    }

    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let tmp_name = format!(
        ".{}.tmp-{}-{}",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("file"),
        pid,
        nanos
    );
    let tmp_path = path
        .parent()
        .map(|p| p.join(&tmp_name))
        .unwrap_or_else(|| Path::new(&tmp_name).to_path_buf());

    let result = (|| -> Result<(), PvError> {
        let mut file = fs::File::create(&tmp_path).map_err(PvError::Io)?;
        file.write_all(data).map_err(PvError::Io)?;
        file.sync_all().map_err(PvError::Io)?;
        fs::rename(&tmp_path, path).map_err(PvError::Io)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp_path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indexes::Index;
    use crate::objects::Blob;
    use crate::pipeline::CommitBuilder;
    use crate::refs_store::RefStore;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir(tag: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_checkout_{tag}_{pid}_{nanos}_{id}"))
    }

    fn make_stores(tag: &str) -> (ObjectStore, RefStore, std::path::PathBuf) {
        let dir = unique_test_dir(tag);
        let obj = ObjectStore::open(&dir).unwrap();
        let refs = RefStore::open(&dir).unwrap();
        (obj, refs, dir)
    }

    fn commit_one_file(
        path: &str,
        content: &[u8],
        obj_store: &ObjectStore,
        ref_store: &RefStore,
    ) -> CommitId {
        let blob = Blob::from_bytes(content.to_vec());
        let mut index = Index::new();
        index.add(path.parse().unwrap(), blob.id.clone());
        obj_store.write(Object::Blob(blob)).unwrap();
        let result = CommitBuilder::new("tester", "test commit", 1_000)
            .commit(&index, obj_store, ref_store)
            .unwrap();
        result.commit_id
    }

    #[test]
    fn checkout_writes_files() {
        let (obj, refs, _) = make_stores("write");
        let commit_id = commit_one_file("hello.txt", b"world", &obj, &refs);
        let dest = unique_test_dir("dest_write");
        let mat =
            Checkout::materialize(&commit_id, &obj, &dest, &CheckoutPolicy::overwrite()).unwrap();
        assert_eq!(mat.files_written, 1);
        assert_eq!(std::fs::read(dest.join("hello.txt")).unwrap(), b"world");
    }

    #[test]
    fn checkout_overwrite_replaces_file() {
        let (obj, refs, _) = make_stores("overwrite");
        let commit_id = commit_one_file("file.txt", b"new content", &obj, &refs);
        let dest = unique_test_dir("dest_overwrite");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("file.txt"), b"old content").unwrap();
        Checkout::materialize(&commit_id, &obj, &dest, &CheckoutPolicy::overwrite()).unwrap();
        assert_eq!(fs::read(dest.join("file.txt")).unwrap(), b"new content");
    }

    #[test]
    fn checkout_safe_fails_on_conflict() {
        let (obj, refs, _) = make_stores("safe");
        let commit_id = commit_one_file("file.txt", b"new content", &obj, &refs);
        let dest = unique_test_dir("dest_safe");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("file.txt"), b"different content").unwrap();
        let result = Checkout::materialize(&commit_id, &obj, &dest, &CheckoutPolicy::safe());
        assert!(result.is_err());
    }

    #[test]
    fn checkout_clean_deletes_untracked() {
        let (obj, refs, _) = make_stores("clean");
        let commit_id = commit_one_file("kept.txt", b"keep", &obj, &refs);
        let dest = unique_test_dir("dest_clean");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("extra.txt"), b"untracked").unwrap();
        let mat = Checkout::materialize(&commit_id, &obj, &dest, &CheckoutPolicy::clean()).unwrap();
        assert_eq!(mat.files_deleted, 1);
        assert!(!dest.join("extra.txt").exists());
    }

    #[test]
    fn unsafe_path_cannot_escape_dest() {
        // SafePath construction prevents traversal, so there is no path to
        // materialize outside of dest. Verify the invariant at the type level.
        assert!("../etc/passwd".parse::<SafePath>().is_err());
    }
}
