//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/runtime_persistence_bundle.rs
use crate::buffer_manager::BufferManager;
use crate::dataset_engine::{Correction, DatasetEntry};
use crate::memory_engine::MemoryEntry;
use crate::orchestrator::{
    AutoImprovementPolicy, AutoImprovementStatus, GovernanceAuditEntry,
    GovernancePersistenceBundle, GovernanceStateSnapshot, ModelRegistry, RuntimeBundleComponents,
    TrainerTriggerEvent,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(default)]
    pub dataset_entries: Vec<DatasetEntry>,
    #[serde(default)]
    pub dataset_corrections: HashMap<String, Vec<Correction>>,
    #[serde(default)]
    pub auto_improvement_policy: Option<AutoImprovementPolicy>,
    #[serde(default)]
    pub auto_improvement_status: AutoImprovementStatus,
    #[serde(default)]
    pub model_registry: ModelRegistry,
    #[serde(default)]
    pub trainer_trigger_events: Vec<TrainerTriggerEvent>,
}

impl RuntimePersistenceBundle {
    pub fn schema_version() -> u32 {
        RUNTIME_BUNDLE_SCHEMA_VERSION
    }

    pub fn has_supported_schema(&self) -> bool {
        self.schema_version == Self::schema_version()
    }

    pub fn from_components(components: RuntimeBundleComponents) -> Self {
        let mut bundle = Self {
            schema_version: RUNTIME_BUNDLE_SCHEMA_VERSION,
            runtime_checksum: String::new(),
            governance: components.governance,
            short_term_memory_entries: components.short_term_memory_entries,
            long_term_memory_entries: components.long_term_memory_entries,
            buffer_manager: components.buffer_manager,
            dataset_entries: components.dataset_entries,
            dataset_corrections: components.dataset_corrections,
            auto_improvement_policy: components.auto_improvement_policy,
            auto_improvement_status: components.auto_improvement_status,
            model_registry: components.model_registry,
            trainer_trigger_events: components.trainer_trigger_events,
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
        recompute_runtime_checksum_from_components(
            self.schema_version,
            self.governance.state.schema_version,
            self.governance.state.state_version,
            &self.governance.state.state_checksum,
            &self.governance.audit_entries,
            &self.governance.snapshots,
            &self.short_term_memory_entries,
            &self.long_term_memory_entries,
            &self.buffer_manager,
            &self.dataset_entries,
            &self.dataset_corrections,
            self.auto_improvement_policy.as_ref(),
            &self.auto_improvement_status,
            &self.model_registry,
            &self.trainer_trigger_events,
        )
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn recompute_runtime_checksum_from_components<'a, 'b, 'c, S, L, T>(
    schema_version: u32,
    governance_state_schema_version: u32,
    governance_state_version: u64,
    governance_state_checksum: &str,
    governance_audit_entries: &[GovernanceAuditEntry],
    governance_snapshots: &[GovernanceStateSnapshot],
    short_term_memory_entries: S,
    long_term_memory_entries: L,
    buffer_manager: &BufferManager,
    dataset_entries: &[DatasetEntry],
    dataset_corrections: &HashMap<String, Vec<Correction>>,
    auto_improvement_policy: Option<&AutoImprovementPolicy>,
    auto_improvement_status: &AutoImprovementStatus,
    model_registry: &ModelRegistry,
    trainer_trigger_events: T,
) -> String
where
    S: IntoIterator<Item = &'a MemoryEntry>,
    L: IntoIterator<Item = &'b MemoryEntry>,
    T: IntoIterator<Item = &'c TrainerTriggerEvent>,
{
    let short_fp = memory_entries_fingerprint(short_term_memory_entries);
    let long_fp = memory_entries_fingerprint(long_term_memory_entries);
    let working_fp = working_buffer_fingerprint(buffer_manager);
    let sessions_fp = session_buffer_fingerprint(buffer_manager);
    let governance_fp = governance_fingerprint(
        governance_state_schema_version,
        governance_state_version,
        governance_state_checksum,
        governance_audit_entries,
        governance_snapshots,
    );
    let dataset_fp = dataset_fingerprint(dataset_entries, dataset_corrections);
    let auto_improvement_fp =
        auto_improvement_fingerprint(auto_improvement_policy, auto_improvement_status);
    let model_registry_fp = model_registry_fingerprint(model_registry);
    let trainer_events_fp = trainer_trigger_events_fingerprint(trainer_trigger_events);

    let material = format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        schema_version,
        governance_fp,
        short_fp,
        long_fp,
        working_fp,
        sessions_fp,
        dataset_fp,
        auto_improvement_fp,
        model_registry_fp,
        trainer_events_fp
    );
    format!("{:016x}", fnv1a64(material.as_bytes()))
}

fn memory_entries_fingerprint<'a, I>(entries: I) -> String
where
    I: IntoIterator<Item = &'a MemoryEntry>,
{
    let mut ordered_entries: Vec<&MemoryEntry> = entries.into_iter().collect();
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
        let values = sessions_buffer.values_ref(session);
        for (value_idx, value) in values.iter().enumerate() {
            if value_idx > 0 {
                fingerprint.push('|');
            }
            fingerprint.push_str(value);
        }
    }
    fingerprint
}

fn governance_fingerprint(
    state_schema_version: u32,
    state_version: u64,
    state_checksum: &str,
    audit_entries: &[GovernanceAuditEntry],
    snapshots: &[GovernanceStateSnapshot],
) -> String {
    let mut fingerprint = String::new();
    if let Ok(()) = write!(
        fingerprint,
        "{}:{}:{}:",
        state_schema_version, state_version, state_checksum
    ) {}
    for (idx, entry) in audit_entries.iter().enumerate() {
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
    for (idx, snapshot) in snapshots.iter().enumerate() {
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

fn dataset_fingerprint(
    entries: &[DatasetEntry],
    corrections: &HashMap<String, Vec<Correction>>,
) -> String {
    let mut ordered_entries: Vec<&DatasetEntry> = entries.iter().collect();
    ordered_entries.sort_by(|a, b| a.id.cmp(&b.id));
    let mut parts = Vec::new();
    for entry in ordered_entries {
        parts.push(format!(
            "{}|{}|{}|{}|{:?}|{:?}|{}|{:?}",
            entry.id,
            entry.task_id.as_str(),
            entry.expert_id.as_str(),
            entry.input,
            entry.outcome,
            entry.score,
            entry.created_at,
            entry.tags
        ));
    }

    let mut ordered_keys: Vec<&str> = corrections.keys().map(String::as_str).collect();
    ordered_keys.sort_unstable();
    for key in ordered_keys {
        if let Some(values) = corrections.get(key) {
            for correction in values {
                parts.push(format!(
                    "corr:{}|{}|{}|{}",
                    correction.entry_id,
                    correction.corrected_output,
                    correction.reason,
                    correction.corrected_at
                ));
            }
        }
    }
    parts.join("::")
}

fn auto_improvement_fingerprint(
    policy: Option<&AutoImprovementPolicy>,
    status: &AutoImprovementStatus,
) -> String {
    let policy_part = if let Some(policy) = policy {
        format!(
            "{}|{}|{:?}|{}|{}|{:?}|{}|{}|{}",
            policy.min_dataset_entries,
            policy.min_success_ratio,
            policy.min_average_score,
            policy.training_build_options.generated_at,
            policy.training_build_options.validation_ratio,
            policy.training_build_options.min_score,
            policy.training_build_options.include_failure_entries,
            policy.training_build_options.include_partial_entries,
            policy.training_build_options.split_seed
        )
    } else {
        "none".to_string()
    };
    let status_part = format!(
        "{}|{}|{:?}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}|{}|{}",
        status.runs_total,
        status.bootstrap_entries_total,
        status.last_bundle_checksum,
        status.last_included_entries,
        status.last_train_samples,
        status.last_validation_samples,
        status.skipped_min_dataset_entries_total,
        status.skipped_min_success_ratio_total,
        status.skipped_min_average_score_total,
        status.skipped_human_review_required_total,
        status.skipped_duplicate_bundle_total,
        status.build_failures_total,
        status.last_skip_reason,
        status.trainer_trigger_delivery_attempts_total,
        status.trainer_trigger_delivery_failures_total,
        status.trainer_trigger_acknowledged_total
    );
    format!("{policy_part}::{status_part}")
}

fn model_registry_fingerprint(registry: &ModelRegistry) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "active={:?}|next={}",
        registry.active_version, registry.next_version
    ));
    for entry in &registry.entries {
        parts.push(format!(
            "{}|{}|{}|{}|{}|{}|{}",
            entry.version,
            entry.training_bundle_checksum,
            entry.included_entries,
            entry.train_samples,
            entry.validation_samples,
            entry.generated_at,
            entry.promoted
        ));
    }
    parts.join("::")
}

fn trainer_trigger_events_fingerprint<'a, I>(events: I) -> String
where
    I: IntoIterator<Item = &'a TrainerTriggerEvent>,
{
    let mut parts = Vec::new();
    for event in events {
        parts.push(format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{:?}",
            event.event_id,
            event.model_version,
            event.training_bundle_checksum,
            event.included_entries,
            event.train_samples,
            event.validation_samples,
            event.generated_at,
            event.delivery_attempts,
            event.last_attempted_at
        ));
    }
    parts.join("::")
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
