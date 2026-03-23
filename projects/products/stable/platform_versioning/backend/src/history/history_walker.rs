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
/// This produces a deterministic newest-first sequence for linear
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
