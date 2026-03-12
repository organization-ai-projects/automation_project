//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/runtime_persistence_bundle.rs
use crate::buffer_manager::BufferManager;
use crate::memory_engine::MemoryEntry;
use crate::orchestrator::GovernancePersistenceBundle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePersistenceBundle {
    pub governance: GovernancePersistenceBundle,
    pub short_term_memory_entries: Vec<MemoryEntry>,
    pub long_term_memory_entries: Vec<MemoryEntry>,
    pub buffer_manager: BufferManager,
}
