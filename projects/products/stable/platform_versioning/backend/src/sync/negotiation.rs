// projects/products/stable/platform_versioning/backend/src/sync/negotiation.rs
use std::collections::{HashSet, VecDeque};

use crate::errors::PvError;
use crate::ids::ObjectId;
use crate::objects::{Object, ObjectStore};
use crate::refs_store::RefStore;
use crate::sync::FetchRequest;

/// Negotiates and collects the set of objects to send in response to a [`FetchRequest`].
pub struct Negotiation;

impl Negotiation {
    /// Returns the list of objects to send to the client.
    ///
    /// Algorithm:
    /// 1. Resolve all `want` refs to their commit ids.
    /// 2. Perform a BFS from each wanted commit, stopping at objects the client
    ///    already has (`have` set).
    /// 3. Return at most `limit` objects (arbitrary order, deterministic within
    ///    a single call for the same inputs).
    pub fn collect(
        request: &FetchRequest,
        object_store: &ObjectStore,
        ref_store: &RefStore,
    ) -> Result<Vec<Object>, PvError> {
        let have: HashSet<ObjectId> = request.have.iter().cloned().collect();
        let limit = request.limit.unwrap_or(usize::MAX);

        let mut want_ids: Vec<ObjectId> = Vec::new();
        for ref_name in &request.want {
            match ref_store.read_ref(ref_name) {
                Ok(target) => {
                    want_ids.push(target.commit_id().as_object_id().clone());
                }
                Err(PvError::RefNotFound(_)) => {
                    // Skip unknown refs gracefully.
                }
                Err(e) => return Err(e),
            }
        }

        let mut result = Vec::new();
        let mut visited: HashSet<ObjectId> = HashSet::new();
        let mut queue: VecDeque<ObjectId> = want_ids.into_iter().collect();

        while let Some(id) = queue.pop_front() {
            if visited.contains(&id) || have.contains(&id) {
                continue;
            }
            visited.insert(id.clone());

            let obj = object_store.read(&id)?;
            match &obj {
                Object::Commit(c) => {
                    if !have.contains(c.tree_id.as_object_id()) {
                        queue.push_back(c.tree_id.as_object_id().clone());
                    }
                    for parent in &c.parent_ids {
                        let pid = parent.as_object_id().clone();
                        if !have.contains(&pid) {
                            queue.push_back(pid);
                        }
                    }
                }
                Object::Tree(t) => {
                    for entry in &t.entries {
                        if !have.contains(&entry.id) {
                            queue.push_back(entry.id.clone());
                        }
                    }
                }
                Object::Blob(_) => {}
            }

            result.push(obj);
            if result.len() >= limit {
                break;
            }
        }

        Ok(result)
    }
}
