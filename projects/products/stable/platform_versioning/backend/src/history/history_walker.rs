// projects/products/stable/platform_versioning/backend/src/history/history_walker.rs
use std::collections::{HashSet, VecDeque};

use crate::errors::PvError;
use crate::history::{HistoryEntry, HistoryPage};
use crate::ids::CommitId;
use crate::objects::{Object, ObjectStore};

/// Traverses commit history from a starting commit.
///
/// # Ordering
/// History is traversed in breadth-first order following first-parent links.
/// This produces a deterministic reverse-chronological sequence for linear
/// histories. Merge commits include all parents in order.
pub struct HistoryWalker<'a> {
    store: &'a ObjectStore,
}

impl<'a> HistoryWalker<'a> {
    /// Creates a new walker backed by `store`.
    pub fn new(store: &'a ObjectStore) -> Self {
        Self { store }
    }

    /// Returns a page of history starting from `start`, with at most `limit` entries.
    ///
    /// Pass the `next_cursor` from the previous [`HistoryPage`] as `start` to
    /// continue paginating.
    pub fn page(&self, start: &CommitId, limit: usize) -> Result<HistoryPage, PvError> {
        if limit == 0 {
            return Ok(HistoryPage {
                entries: vec![],
                next_cursor: Some(start.clone()),
            });
        }

        let mut entries = Vec::with_capacity(limit);
        let mut queue: VecDeque<CommitId> = VecDeque::new();
        let mut visited: HashSet<CommitId> = HashSet::new();

        queue.push_back(start.clone());

        while let Some(current_id) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            let obj = self.store.read(current_id.as_object_id())?;
            let commit = match obj {
                Object::Commit(c) => c,
                _ => return Err(PvError::Internal(format!("{current_id} is not a commit"))),
            };

            for parent in &commit.parent_ids {
                if !visited.contains(parent) {
                    queue.push_back(parent.clone());
                }
            }

            entries.push(HistoryEntry {
                commit_id: current_id,
                author: commit.author,
                message: commit.message,
                timestamp_secs: commit.timestamp_secs,
                parent_ids: commit.parent_ids,
            });

            if entries.len() == limit {
                // Return the next start point if the queue is non-empty.
                let next_cursor = queue.front().cloned();
                return Ok(HistoryPage {
                    entries,
                    next_cursor,
                });
            }
        }

        Ok(HistoryPage {
            entries,
            next_cursor: None,
        })
    }
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
        std::env::temp_dir().join(format!("pv_hist_{tag}_{pid}_{nanos}_{id}"))
    }

    fn make_stores(tag: &str) -> (ObjectStore, RefStore) {
        let dir = unique_test_dir(tag);
        let obj = ObjectStore::open(&dir).unwrap();
        let refs = RefStore::open(&dir).unwrap();
        (obj, refs)
    }

    fn do_commit(
        path: &str,
        content: &[u8],
        ts: u64,
        obj: &ObjectStore,
        refs: &RefStore,
    ) -> CommitId {
        let blob = Blob::from_bytes(content.to_vec());
        let mut idx = Index::new();
        idx.add(path.parse().unwrap(), blob.id.clone());
        obj.write(Object::Blob(blob)).unwrap();
        CommitBuilder::new("user", "msg", ts)
            .commit(&idx, obj, refs)
            .unwrap()
            .commit_id
    }

    #[test]
    fn single_commit_history() {
        let (obj, refs) = make_stores("single");
        let id = do_commit("a.txt", b"data", 1, &obj, &refs);
        let walker = HistoryWalker::new(&obj);
        let page = walker.page(&id, 10).unwrap();
        assert_eq!(page.entries.len(), 1);
        assert!(page.next_cursor.is_none());
    }

    #[test]
    fn linear_history_pagination() {
        let (obj, refs) = make_stores("paginate");
        let _id1 = do_commit("a.txt", b"1", 1, &obj, &refs);
        let _id2 = do_commit("b.txt", b"2", 2, &obj, &refs);
        let id3 = do_commit("c.txt", b"3", 3, &obj, &refs);

        let walker = HistoryWalker::new(&obj);
        let page1 = walker.page(&id3, 2).unwrap();
        assert_eq!(page1.entries.len(), 2);
        assert!(page1.next_cursor.is_some());

        let page2 = walker.page(&page1.next_cursor.unwrap(), 2).unwrap();
        assert_eq!(page2.entries.len(), 1);
        assert!(page2.next_cursor.is_none());
    }

    #[test]
    fn invalid_start_returns_error() {
        let (obj, _refs) = make_stores("invalid");
        let bad_id = CommitId::from(crate::ids::ObjectId::from_bytes(&[0xddu8; 32]));
        let walker = HistoryWalker::new(&obj);
        let result = walker.page(&bad_id, 10);
        assert!(result.is_err());
    }
}
