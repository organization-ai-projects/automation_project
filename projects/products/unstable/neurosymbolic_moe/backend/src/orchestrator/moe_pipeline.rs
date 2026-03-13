//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/moe_pipeline.rs
use crate::aggregator::OutputAggregator;
use crate::buffer_manager::BufferManager;
use crate::dataset_engine::{
    DatasetStore, DatasetTrainingBuildOptions, DatasetTrainingBundle, DatasetTrainingProvenance,
    DatasetTrainingShard, TraceConverter,
};
use crate::evaluation_engine::EvaluationEngine;
use crate::expert_registry::ExpertRegistry;
use crate::feedback_engine::{FeedbackEntry, FeedbackStore};
use crate::memory_engine::{LongTermMemory, MemoryEntry, MemoryStore, ShortTermMemory};
use crate::moe_core::{Expert, MoeError};
use crate::orchestrator::ContinuousImprovementReport;
use crate::orchestrator::{
    ArbitrationMode, ContinuousGovernancePolicy, GovernanceAuditEntry, GovernanceImportPolicy,
    GovernanceState, GovernanceStateSnapshot, ImportTelemetry,
};
use crate::policy_guard::{Policy, PolicyGuard};
use crate::retrieval_engine::{ContextAssembler, Retriever};
use crate::router::Router;
use crate::trace_logger::TraceLogger;

const MAX_RUNTIME_BUNDLE_JSON_BYTES: usize = 16 * 1024 * 1024;
const MAX_GOVERNANCE_STATE_JSON_BYTES: usize = 4 * 1024 * 1024;
const MAX_GOVERNANCE_BUNDLE_JSON_BYTES: usize = 16 * 1024 * 1024;
const MAX_RUNTIME_BUNDLE_TOTAL_MEMORY_ENTRIES: usize = 10_000;
const MAX_RUNTIME_BUNDLE_WORKING_ENTRIES: usize = 10_000;
const MAX_RUNTIME_BUNDLE_SESSION_COUNT: usize = 2_000;
const MAX_RUNTIME_BUNDLE_SESSION_VALUES_TOTAL: usize = 20_000;
const MAX_TRAINING_DATASET_BUNDLE_JSON_BYTES: usize = 64 * 1024 * 1024;
const MAX_TRAINING_DATASET_SHARDS_JSON_BYTES: usize = 128 * 1024 * 1024;

mod execution;
mod governance_runtime;
mod persistence;

pub struct MoePipeline {
    pub(super) registry: ExpertRegistry,
    pub(super) router: Box<dyn Router>,
    pub(super) retriever: Box<dyn Retriever>,
    pub(super) context_assembler: ContextAssembler,
    pub(super) short_term_memory: ShortTermMemory,
    pub(super) long_term_memory: LongTermMemory,
    pub(super) buffer_manager: BufferManager,
    pub(super) aggregator: OutputAggregator,
    pub(super) arbitration_mode: ArbitrationMode,
    pub(super) fallback_on_expert_error: bool,
    pub(super) enable_task_metadata_chain: bool,
    pub(super) continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    pub(super) governance_import_policy: GovernanceImportPolicy,
    pub(super) policy_guard: PolicyGuard,
    pub(super) trace_logger: TraceLogger,
    pub(super) evaluation: EvaluationEngine,
    pub(super) evaluation_baseline: Option<EvaluationEngine>,
    pub(super) last_continuous_improvement_report: Option<ContinuousImprovementReport>,
    pub(super) governance_state_version: u64,
    pub(super) governance_audit_entries: Vec<GovernanceAuditEntry>,
    pub(super) max_governance_audit_entries: usize,
    pub(super) governance_state_snapshots: Vec<GovernanceStateSnapshot>,
    pub(super) max_governance_state_snapshots: usize,
    pub(super) import_telemetry: ImportTelemetry,
    pub(super) feedback_store: FeedbackStore,
    pub(super) dataset_store: DatasetStore,
    pub(super) trace_converter: TraceConverter,
}

impl MoePipeline {
    pub fn register_expert(&mut self, expert: Box<dyn Expert>) -> Result<(), MoeError> {
        self.registry.register(expert)
    }

    pub fn add_policy(&mut self, policy: Policy) {
        self.policy_guard.add_policy(policy);
    }

    pub fn remember_short_term(&mut self, entry: MemoryEntry) -> Result<(), MoeError> {
        self.short_term_memory.store(entry)
    }

    pub fn remember_long_term(&mut self, entry: MemoryEntry) -> Result<(), MoeError> {
        self.long_term_memory.store(entry)
    }

    pub fn put_session_buffer(
        &mut self,
        session_id: &str,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.buffer_manager
            .sessions_mut()
            .put(session_id, key, value);
    }

    pub fn registry(&self) -> &ExpertRegistry {
        &self.registry
    }

    pub fn trace_logger(&self) -> &TraceLogger {
        &self.trace_logger
    }

    pub fn evaluation(&self) -> &EvaluationEngine {
        &self.evaluation
    }

    pub fn feedback_store(&self) -> &FeedbackStore {
        &self.feedback_store
    }

    pub fn dataset_store(&self) -> &DatasetStore {
        &self.dataset_store
    }

    pub fn import_telemetry_snapshot(&self) -> ImportTelemetry {
        self.import_telemetry.clone()
    }

    pub fn export_training_dataset_bundle(
        &self,
        options: &DatasetTrainingBuildOptions,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        self.dataset_store
            .build_training_bundle_with_provenance(options, self.dataset_provenance())
    }

    pub fn export_training_dataset_bundle_json(
        &self,
        options: &DatasetTrainingBuildOptions,
    ) -> Result<String, MoeError> {
        let bundle = self.export_training_dataset_bundle(options)?;
        let payload = common_json::json::to_json_string_pretty(&bundle).map_err(|err| {
            MoeError::DatasetError(format!(
                "training dataset bundle serialization failed: {err}"
            ))
        })?;
        if payload.len() > MAX_TRAINING_DATASET_BUNDLE_JSON_BYTES {
            return Err(MoeError::DatasetError(format!(
                "training dataset bundle payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_TRAINING_DATASET_BUNDLE_JSON_BYTES
            )));
        }
        Ok(payload)
    }

    pub fn export_training_dataset_shards(
        &self,
        options: &DatasetTrainingBuildOptions,
        max_samples_per_shard: usize,
    ) -> Result<Vec<DatasetTrainingShard>, MoeError> {
        let bundle = self.export_training_dataset_bundle(options)?;
        DatasetStore::shard_training_bundle(&bundle, max_samples_per_shard)
    }

    pub fn export_training_dataset_shards_json(
        &self,
        options: &DatasetTrainingBuildOptions,
        max_samples_per_shard: usize,
    ) -> Result<String, MoeError> {
        let shards = self.export_training_dataset_shards(options, max_samples_per_shard)?;
        let payload = common_json::json::to_json_string_pretty(&shards).map_err(|err| {
            MoeError::DatasetError(format!(
                "training dataset shard serialization failed: {err}"
            ))
        })?;
        if payload.len() > MAX_TRAINING_DATASET_SHARDS_JSON_BYTES {
            return Err(MoeError::DatasetError(format!(
                "training dataset shard payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_TRAINING_DATASET_SHARDS_JSON_BYTES
            )));
        }
        Ok(payload)
    }

    pub fn rebuild_training_dataset_bundle_from_shards(
        &self,
        shards: &[DatasetTrainingShard],
    ) -> Result<DatasetTrainingBundle, MoeError> {
        DatasetStore::rebuild_training_bundle_from_shards(shards)
    }

    pub fn rebuild_training_dataset_bundle_from_shards_json(
        &self,
        payload: &str,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        if payload.len() > MAX_TRAINING_DATASET_SHARDS_JSON_BYTES {
            return Err(MoeError::DatasetError(format!(
                "training dataset shard payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_TRAINING_DATASET_SHARDS_JSON_BYTES
            )));
        }
        let shards: Vec<DatasetTrainingShard> =
            common_json::json::from_json_str(payload).map_err(|err| {
                MoeError::DatasetError(format!(
                    "training dataset shard deserialization failed: {err}"
                ))
            })?;
        self.rebuild_training_dataset_bundle_from_shards(&shards)
    }

    pub fn preview_training_dataset_bundle_json(
        &self,
        payload: &str,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        if payload.len() > MAX_TRAINING_DATASET_BUNDLE_JSON_BYTES {
            return Err(MoeError::DatasetError(format!(
                "training dataset bundle payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_TRAINING_DATASET_BUNDLE_JSON_BYTES
            )));
        }
        let mut bundle: DatasetTrainingBundle =
            common_json::json::from_json_str(payload).map_err(|err| {
                MoeError::DatasetError(format!(
                    "training dataset bundle deserialization failed: {err}"
                ))
            })?;
        bundle.ensure_checksum();
        DatasetStore::validate_training_bundle(&bundle)?;
        Ok(bundle)
    }

    pub fn preview_training_dataset_shards_json(
        &self,
        payload: &str,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        self.rebuild_training_dataset_bundle_from_shards_json(payload)
    }

    fn dataset_provenance(&self) -> DatasetTrainingProvenance {
        let governance_state = self.export_governance_state();
        let runtime_bundle = self.export_runtime_bundle();
        DatasetTrainingProvenance {
            generator: "neurosymbolic_moe_backend.orchestrator".to_string(),
            governance_state_version: governance_state.state_version,
            governance_state_checksum: governance_state.state_checksum,
            runtime_bundle_checksum: runtime_bundle.runtime_checksum,
            dataset_entry_count: self.dataset_store.count(),
        }
    }

    pub fn add_feedback(&mut self, entry: FeedbackEntry) {
        self.feedback_store.add(entry);
    }

    pub fn capture_evaluation_baseline(&mut self) {
        self.evaluation_baseline = Some(self.evaluation.clone());
    }

    pub fn last_continuous_improvement_report(&self) -> Option<&ContinuousImprovementReport> {
        self.last_continuous_improvement_report.as_ref()
    }

    pub fn has_evaluation_baseline(&self) -> bool {
        self.evaluation_baseline.is_some()
    }

    pub fn approve_pending_human_review_and_promote(&mut self) -> bool {
        if self
            .last_continuous_improvement_report
            .as_ref()
            .is_some_and(|report| report.requires_human_review)
        {
            self.capture_evaluation_baseline();
            if let Some(report) = self.last_continuous_improvement_report.as_mut() {
                report.requires_human_review = false;
            }
            self.record_governance_audit("human approval promotion");
            true
        } else {
            false
        }
    }

    pub fn export_governance_state(&self) -> GovernanceState {
        GovernanceState::from_components(
            self.governance_state_version,
            self.continuous_governance_policy.clone(),
            self.evaluation_baseline.clone(),
            self.last_continuous_improvement_report.clone(),
        )
    }
}
