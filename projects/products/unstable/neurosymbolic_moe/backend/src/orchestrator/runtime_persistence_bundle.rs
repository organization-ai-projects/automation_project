//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/runtime_persistence_bundle.rs
use crate::buffer_manager::BufferManager;
use crate::memory_engine::MemoryEntry;
use crate::orchestrator::GovernancePersistenceBundle;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

const RUNTIME_BUNDLE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePersistenceBundle {
    #[serde(default = "RuntimePersistenceBundle::schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub runtime_checksum: String,
    pub governance: GovernancePersistenceBundle,
    pub short_term_memory_entries: Vec<MemoryEntry>,
    pub long_term_memory_entries: Vec<MemoryEntry>,
    pub buffer_manager: BufferManager,
}

impl RuntimePersistenceBundle {
    pub fn schema_version() -> u32 {
        RUNTIME_BUNDLE_SCHEMA_VERSION
    }

    pub fn has_supported_schema(&self) -> bool {
        self.schema_version == Self::schema_version()
    }

    pub fn from_components(
        governance: GovernancePersistenceBundle,
        short_term_memory_entries: Vec<MemoryEntry>,
        long_term_memory_entries: Vec<MemoryEntry>,
        buffer_manager: BufferManager,
    ) -> Self {
        let mut bundle = Self {
            schema_version: RUNTIME_BUNDLE_SCHEMA_VERSION,
            runtime_checksum: String::new(),
            governance,
            short_term_memory_entries,
            long_term_memory_entries,
            buffer_manager,
        };
        bundle.runtime_checksum = bundle.recompute_checksum();
        bundle
    }

    pub fn ensure_checksum(&mut self) {
        if self.runtime_checksum.is_empty() {
            self.runtime_checksum = self.recompute_checksum();
        }
    }

    pub fn verify_checksum(&self) -> bool {
        !self.runtime_checksum.is_empty() && self.runtime_checksum == self.recompute_checksum()
    }

    pub fn recompute_checksum(&self) -> String {
        let short_fp = memory_entries_fingerprint(&self.short_term_memory_entries);
        let long_fp = memory_entries_fingerprint(&self.long_term_memory_entries);
        let working_fp = working_buffer_fingerprint(&self.buffer_manager);
        let sessions_fp = session_buffer_fingerprint(&self.buffer_manager);
        let governance_fp = governance_fingerprint(&self.governance);

        let material = format!(
            "{}:{}:{}:{}:{}:{}",
            self.schema_version, governance_fp, short_fp, long_fp, working_fp, sessions_fp
        );
        format!("{:016x}", fnv1a64(material.as_bytes()))
    }
}

fn memory_entries_fingerprint(entries: &[MemoryEntry]) -> String {
    let mut ordered_entries: Vec<&MemoryEntry> = entries.iter().collect();
    ordered_entries.sort_by(|a, b| {
        a.id.cmp(&b.id)
            .then(a.content.cmp(&b.content))
            .then(a.created_at.cmp(&b.created_at))
    });
    let mut fingerprint = String::new();
    for (idx, entry) in ordered_entries.iter().enumerate() {
        if idx > 0 {
            fingerprint.push(';');
        }
        let mut tags: Vec<&str> = entry.tags.iter().map(String::as_str).collect();
        tags.sort_unstable();
        let mut metadata: Vec<(&str, &str)> = entry
            .metadata
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        metadata.sort_unstable_by(|a, b| a.0.cmp(b.0));
        if let Ok(()) = write!(
            fingerprint,
            "{}|{}|{:?}|{}|{:?}|{}|{:?}|",
            entry.id,
            entry.content,
            tags,
            entry.created_at,
            entry.expires_at,
            entry.relevance,
            entry.memory_type
        ) {}
        for (metadata_idx, (key, value)) in metadata.iter().enumerate() {
            if metadata_idx > 0 {
                fingerprint.push(',');
            }
            if let Ok(()) = write!(fingerprint, "{key}={value}") {}
        }
    }
    fingerprint
}

fn working_buffer_fingerprint(buffer_manager: &BufferManager) -> String {
    let working = buffer_manager.working();
    let mut keys = working.keys();
    keys.sort_unstable();
    let mut fingerprint = String::new();
    for key in keys {
        if let Some(entry) = working.get(key) {
            if !fingerprint.is_empty() {
                fingerprint.push(';');
            }
            if let Ok(()) = write!(fingerprint, "{}={}", entry.key, entry.value) {}
        }
    }
    fingerprint
}

fn session_buffer_fingerprint(buffer_manager: &BufferManager) -> String {
    let sessions_buffer = buffer_manager.sessions();
    let mut sessions = sessions_buffer.list_sessions();
    sessions.sort_unstable();
    let mut fingerprint = String::new();
    for session in sessions {
        if !fingerprint.is_empty() {
            fingerprint.push(';');
        }
        if let Ok(()) = write!(fingerprint, "{}:", session) {}
        let values = sessions_buffer.values(session);
        for (value_idx, value) in values.iter().enumerate() {
            if value_idx > 0 {
                fingerprint.push('|');
            }
            fingerprint.push_str(value);
        }
    }
    fingerprint
}

fn governance_fingerprint(governance: &GovernancePersistenceBundle) -> String {
    let mut fingerprint = String::new();
    if let Ok(()) = write!(
        fingerprint,
        "{}:{}:{}:",
        governance.state.schema_version,
        governance.state.state_version,
        governance.state.state_checksum
    ) {}
    for (idx, entry) in governance.audit_entries.iter().enumerate() {
        if idx > 0 {
            fingerprint.push('|');
        }
        if let Ok(()) = write!(
            fingerprint,
            "{}:{}:{}",
            entry.version, entry.checksum, entry.reason
        ) {}
    }
    fingerprint.push_str("::");
    for (idx, snapshot) in governance.snapshots.iter().enumerate() {
        if idx > 0 {
            fingerprint.push('|');
        }
        if let Ok(()) = write!(
            fingerprint,
            "{}:{}:{}",
            snapshot.version, snapshot.reason, snapshot.state.state_checksum
        ) {}
    }
    fingerprint
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
