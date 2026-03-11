use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::moe_core::TaskId;

use super::buffer_entry::BufferEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingBuffer {
    entries: HashMap<String, BufferEntry>,
    capacity: usize,
}

impl WorkingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            capacity,
        }
    }

    pub fn put(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        task_id: Option<TaskId>,
    ) {
        let key = key.into();
        if self.entries.len() >= self.capacity && !self.entries.contains_key(&key) {
            self.evict_oldest();
        }
        let entry = BufferEntry {
            key: key.clone(),
            value: value.into(),
            created_at: self.next_timestamp(),
            task_id,
            session_id: None,
        };
        self.entries.insert(key, entry);
    }

    pub fn get(&self, key: &str) -> Option<&BufferEntry> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<BufferEntry> {
        self.entries.remove(key)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn keys(&self) -> Vec<&str> {
        self.entries.keys().map(|k| k.as_str()).collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
        {
            self.entries.remove(&oldest_key);
        }
    }

    fn next_timestamp(&self) -> u64 {
        self.entries
            .values()
            .map(|e| e.created_at)
            .max()
            .unwrap_or(0)
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_and_get() {
        let mut buf = WorkingBuffer::new(10);
        buf.put("k1", "v1", None);
        let entry = buf.get("k1").unwrap();
        assert_eq!(entry.value, "v1");
        assert_eq!(buf.count(), 1);
    }

    #[test]
    fn capacity_eviction() {
        let mut buf = WorkingBuffer::new(2);
        buf.put("k1", "v1", None);
        buf.put("k2", "v2", None);
        buf.put("k3", "v3", None);
        assert_eq!(buf.count(), 2);
        // k1 was oldest and should be evicted
        assert!(buf.get("k1").is_none());
        assert!(buf.get("k2").is_some());
        assert!(buf.get("k3").is_some());
    }
}
