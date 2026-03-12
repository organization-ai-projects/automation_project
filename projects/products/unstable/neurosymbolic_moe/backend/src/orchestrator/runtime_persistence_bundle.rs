//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/runtime_persistence_bundle.rs
use crate::buffer_manager::BufferManager;
use crate::memory_engine::MemoryEntry;
use crate::orchestrator::GovernancePersistenceBundle;
use serde::{Deserialize, Serialize};

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
    let mut entries = entries.to_vec();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    entries
        .iter()
        .map(|entry| {
            let mut tags = entry.tags.clone();
            tags.sort();
            let mut metadata: Vec<(&str, &str)> = entry
                .metadata
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            metadata.sort_by(|a, b| a.0.cmp(b.0));
            format!(
                "{}|{}|{:?}|{}|{:?}|{}|{:?}|{}",
                entry.id,
                entry.content,
                tags,
                entry.created_at,
                entry.expires_at,
                entry.relevance,
                entry.memory_type,
                metadata
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn working_buffer_fingerprint(buffer_manager: &BufferManager) -> String {
    let mut keys = buffer_manager.working().keys();
    keys.sort_unstable();
    keys.into_iter()
        .filter_map(|key| {
            buffer_manager
                .working()
                .get(key)
                .map(|entry| format!("{}={}", entry.key, entry.value))
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn session_buffer_fingerprint(buffer_manager: &BufferManager) -> String {
    let mut sessions = buffer_manager.sessions().list_sessions();
    sessions.sort_unstable();
    sessions
        .into_iter()
        .map(|session| {
            format!(
                "{}:{}",
                session,
                buffer_manager.sessions().values(session).join("|")
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn governance_fingerprint(governance: &GovernancePersistenceBundle) -> String {
    let audit_fp = governance
        .audit_entries
        .iter()
        .map(|entry| format!("{}:{}:{}", entry.version, entry.checksum, entry.reason))
        .collect::<Vec<_>>()
        .join("|");
    let snapshot_fp = governance
        .snapshots
        .iter()
        .map(|snapshot| {
            format!(
                "{}:{}:{}",
                snapshot.version, snapshot.reason, snapshot.state.state_checksum
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "{}:{}:{}:{audit_fp}::{snapshot_fp}",
        governance.state.schema_version,
        governance.state.state_version,
        governance.state.state_checksum
    )
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
