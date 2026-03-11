use std::collections::HashMap;

use crate::moe_core::MoeError;

use super::memory_entry::{MemoryEntry, MemoryQuery};
use super::memory_store::MemoryStore;

#[derive(Debug, Clone)]
pub struct LongTermMemory {
    entries: HashMap<String, MemoryEntry>,
}

impl LongTermMemory {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl Default for LongTermMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore for LongTermMemory {
    fn store(&mut self, entry: MemoryEntry) -> Result<(), MoeError> {
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

    fn make_entry(id: &str, tags: Vec<&str>) -> MemoryEntry {
        MemoryEntry {
            id: id.to_string(),
            content: format!("content-{id}"),
            tags: tags.into_iter().map(String::from).collect(),
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::LongTerm,
            relevance: 1.0,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn store_and_retrieve() {
        let mut mem = LongTermMemory::new();
        mem.store(make_entry("e1", vec!["a"])).unwrap();
        mem.store(make_entry("e2", vec!["b"])).unwrap();
        assert_eq!(mem.count(), 2);

        let query = MemoryQuery {
            tags: None,
            memory_type: None,
            min_relevance: None,
            max_results: 10,
            include_expired: true,
        };
        let results = mem.retrieve(&query).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_tags() {
        let mut mem = LongTermMemory::new();
        mem.store(make_entry("e1", vec!["rust"])).unwrap();
        mem.store(make_entry("e2", vec!["python"])).unwrap();

        let query = MemoryQuery {
            tags: Some(vec!["rust".to_string()]),
            memory_type: None,
            min_relevance: None,
            max_results: 10,
            include_expired: true,
        };
        let results = mem.retrieve(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "e1");
    }
}
