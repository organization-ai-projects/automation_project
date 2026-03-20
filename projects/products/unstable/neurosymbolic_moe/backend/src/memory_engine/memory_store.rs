use crate::moe_core::MoeError;

use super::{MemoryEntry, MemoryQuery};

pub trait MemoryStore {
    fn store(&mut self, entry: MemoryEntry) -> Result<(), MoeError>;
    fn retrieve(&self, query: &MemoryQuery) -> Result<Vec<&MemoryEntry>, MoeError>;
    fn remove(&mut self, id: &str) -> Option<MemoryEntry>;
    fn expire(&mut self, current_time: u64) -> usize;
    fn count(&self) -> usize;
}
