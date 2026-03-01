use crate::relations::memory_entry::MemoryEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryLog {
    pub entries: Vec<MemoryEntry>,
}

impl MemoryLog {
    #[allow(dead_code)]
    pub fn push(&mut self, entry: MemoryEntry) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
