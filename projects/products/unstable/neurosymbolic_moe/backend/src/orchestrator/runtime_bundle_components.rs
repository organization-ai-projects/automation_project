//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/runtime_bundle_components.rs
use crate::buffer_manager::BufferManager;
use crate::dataset_engine::{Correction, DatasetEntry};
use crate::memory_engine::MemoryEntry;
use crate::orchestrator::{
    AutoImprovementPolicy, AutoImprovementStatus, GovernancePersistenceBundle,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RuntimeBundleComponents {
    pub governance: GovernancePersistenceBundle,
    pub short_term_memory_entries: Vec<MemoryEntry>,
    pub long_term_memory_entries: Vec<MemoryEntry>,
    pub buffer_manager: BufferManager,
    pub dataset_entries: Vec<DatasetEntry>,
    pub dataset_corrections: HashMap<String, Vec<Correction>>,
    pub auto_improvement_policy: Option<AutoImprovementPolicy>,
    pub auto_improvement_status: AutoImprovementStatus,
}
