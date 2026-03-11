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
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_engine::MemoryType;

    fn make_entry(id: &str, created_at: u64, expires_at: Option<u64>) -> MemoryEntry {
        MemoryEntry {
            id: id.to_string(),
            content: format!("content-{id}"),
            tags: vec!["tag1".to_string()],
            created_at,
            expires_at,
            memory_type: MemoryType::ShortTerm,
            relevance: 1.0,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn store_and_retrieve() {
        let mut mem = ShortTermMemory::new(10);
        mem.store(make_entry("e1", 1, None)).unwrap();
        mem.store(make_entry("e2", 2, None)).unwrap();
        assert_eq!(mem.count(), 2);

        let query = MemoryQuery {
            tags: Some(vec!["tag1".to_string()]),
            memory_type: None,
            min_relevance: None,
            max_results: 10,
            include_expired: true,
        };
        let results = mem.retrieve(&query).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn capacity_eviction() {
        let mut mem = ShortTermMemory::new(2);
        mem.store(make_entry("e1", 1, None)).unwrap();
        mem.store(make_entry("e2", 2, None)).unwrap();
        mem.store(make_entry("e3", 3, None)).unwrap();
        assert_eq!(mem.count(), 2);
        // Oldest (e1) should have been evicted
        assert!(mem.remove("e1").is_none());
    }

    #[test]
    fn expiration() {
        let mut mem = ShortTermMemory::new(10);
        mem.store(make_entry("e1", 1, Some(100))).unwrap();
        mem.store(make_entry("e2", 2, Some(200))).unwrap();
        mem.store(make_entry("e3", 3, None)).unwrap();

        let expired = mem.expire(150);
        assert_eq!(expired, 1);
        assert_eq!(mem.count(), 2);
    }
}
