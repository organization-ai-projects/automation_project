//! projects/products/unstable/neurosymbolic_moe/backend/src/memory_engine/long_term_memory.rs
use std::collections::HashMap;

use crate::moe_core::MoeError;

use super::memory_entry::{MemoryEntry, MemoryQuery};
use super::memory_store::MemoryStore;

#[derive(Debug, Clone)]
pub struct ShortTermMemory {
    entries: HashMap<String, MemoryEntry>,
    capacity: usize,
}

impl ShortTermMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            capacity,
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_id) = self
            .entries
            .values()
            .min_by_key(|e| e.created_at)
            .map(|e| e.id.clone())
        {
            self.entries.remove(&oldest_id);
        }
    }
}

impl MemoryStore for ShortTermMemory {
    fn store(&mut self, entry: MemoryEntry) -> Result<(), MoeError> {
        if self.entries.len() >= self.capacity && !self.entries.contains_key(&entry.id) {
            self.evict_oldest();
        }
        self.entries.insert(entry.id.clone(), entry);
        Ok(())
    }

    fn retrieve(&self, query: &MemoryQuery) -> Result<Vec<&MemoryEntry>, MoeError> {
        let now = query.current_time.unwrap_or(0);

        let results: Vec<&MemoryEntry> = self
            .entries
            .values()
            .filter(|entry| {
                if !query.include_expired
                    && let Some(expires_at) = entry.expires_at
                    && expires_at <= now
                {
                    return false;
                }

                if let Some(ref tags) = query.tags
                    && !tags.iter().any(|t| entry.tags.contains(t))
                {
                    return false;
                }

                if let Some(ref mem_type) = query.memory_type
                    && &entry.memory_type != mem_type
                {
                    return false;
                }

                if let Some(min_rel) = query.min_relevance
                    && entry.relevance < min_rel
                {
                    return false;
                }

                true
            })
            .take(query.max_results)
            .collect();

        Ok(results)
    }

    fn remove(&mut self, id: &str) -> Option<MemoryEntry> {
        self.entries.remove(id)
    }

    fn expire(&mut self, current_time: u64) -> usize {
        let expired_ids: Vec<String> = self
            .entries
            .values()
            .filter(|e| matches!(e.expires_at, Some(t) if t <= current_time))
            .map(|e| e.id.clone())
            .collect();
        let count = expired_ids.len();
        for id in expired_ids {
            self.entries.remove(&id);
        }
        count
    }

    fn count(&self) -> usize {
        self.entries.len()
    }
}
