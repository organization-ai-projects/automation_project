//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/moe_pipeline.rs
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
use crate::orchestrator::import_journal::ImportJournal;
use crate::orchestrator::{
    ArbitrationMode, AutoImprovementPolicy, AutoImprovementStatus, ContinuousGovernancePolicy,
    GovernanceAuditEntry, GovernanceImportPolicy, GovernanceState, GovernanceStateSnapshot,
    ImportTelemetry, ModelRegistry, TrainerTriggerEvent,
};
use crate::policy_guard::{Policy, PolicyGuard};
use crate::retrieval_engine::{ContextAssembler, Retriever};
use crate::router::Router;
use crate::trace_logger::TraceLogger;

pub(in crate::orchestrator::pipeline_moe) const MAX_RUNTIME_BUNDLE_JSON_BYTES: usize =
    16 * 1024 * 1024;
pub(in crate::orchestrator::pipeline_moe) const MAX_GOVERNANCE_STATE_JSON_BYTES: usize =
    4 * 1024 * 1024;
pub(in crate::orchestrator::pipeline_moe) const MAX_GOVERNANCE_BUNDLE_JSON_BYTES: usize =
    16 * 1024 * 1024;
pub(in crate::orchestrator::pipeline_moe) const MAX_RUNTIME_BUNDLE_TOTAL_MEMORY_ENTRIES: usize =
    10_000;
pub(in crate::orchestrator::pipeline_moe) const MAX_RUNTIME_BUNDLE_WORKING_ENTRIES: usize = 10_000;
pub(in crate::orchestrator::pipeline_moe) const MAX_RUNTIME_BUNDLE_SESSION_COUNT: usize = 2_000;
pub(in crate::orchestrator::pipeline_moe) const MAX_RUNTIME_BUNDLE_SESSION_VALUES_TOTAL: usize =
    20_000;
const MAX_TRAINING_DATASET_BUNDLE_JSON_BYTES: usize = 64 * 1024 * 1024;
const MAX_TRAINING_DATASET_SHARDS_JSON_BYTES: usize = 128 * 1024 * 1024;

pub struct MoePipeline {
    pub(in crate::orchestrator) registry: ExpertRegistry,
    pub(in crate::orchestrator) router: Box<dyn Router>,
    pub(in crate::orchestrator) retriever: Box<dyn Retriever>,
    pub(in crate::orchestrator) context_assembler: ContextAssembler,
    pub(in crate::orchestrator) short_term_memory: ShortTermMemory,
    pub(in crate::orchestrator) long_term_memory: LongTermMemory,
    pub(in crate::orchestrator) buffer_manager: BufferManager,
    pub(in crate::orchestrator) aggregator: OutputAggregator,
    pub(in crate::orchestrator) arbitration_mode: ArbitrationMode,
    pub(in crate::orchestrator) fallback_on_expert_error: bool,
    pub(in crate::orchestrator) enable_task_metadata_chain: bool,
    pub(in crate::orchestrator) continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    pub(in crate::orchestrator) governance_import_policy: GovernanceImportPolicy,
    pub(in crate::orchestrator) policy_guard: PolicyGuard,
    pub(in crate::orchestrator) trace_logger: TraceLogger,
    pub(in crate::orchestrator) evaluation: EvaluationEngine,
    pub(in crate::orchestrator) evaluation_baseline: Option<EvaluationEngine>,
    pub(in crate::orchestrator) last_continuous_improvement_report:
        Option<ContinuousImprovementReport>,
    pub(in crate::orchestrator) governance_state_version: u64,
    pub(in crate::orchestrator) governance_audit_entries: Vec<GovernanceAuditEntry>,
    pub(in crate::orchestrator) max_governance_audit_entries: usize,
    pub(in crate::orchestrator) governance_state_snapshots: Vec<GovernanceStateSnapshot>,
    pub(in crate::orchestrator) max_governance_state_snapshots: usize,
    pub(in crate::orchestrator) import_telemetry: ImportTelemetry,
    pub(in crate::orchestrator) import_journal: ImportJournal,
    pub(in crate::orchestrator) feedback_store: FeedbackStore,
    pub(in crate::orchestrator) dataset_store: DatasetStore,
    pub(in crate::orchestrator) trace_converter: TraceConverter,
    pub(in crate::orchestrator) auto_improvement_policy: Option<AutoImprovementPolicy>,
    pub(in crate::orchestrator) auto_improvement_status: AutoImprovementStatus,
    pub(in crate::orchestrator) model_registry: ModelRegistry,
    pub(in crate::orchestrator) trainer_trigger_events: Vec<TrainerTriggerEvent>,
    pub(in crate::orchestrator) max_trainer_trigger_events: usize,
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

    pub fn import_journal_events_total(&self) -> u64 {
        self.import_journal.events_total()
    }

    pub fn import_journal_deduplicated_replays_total(&self) -> u64 {
        self.import_journal.deduplicated_replays_total()
    }

    pub fn import_journal_parse_failures_total(&self) -> u64 {
        self.import_journal.parse_failures_total()
    }

    pub fn import_journal_rejections_total(&self) -> u64 {
        self.import_journal.rejections_total()
    }

    pub fn import_journal_successful_imports_total(&self) -> u64 {
        self.import_journal.successful_imports_total()
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

    pub(in crate::orchestrator::pipeline_moe) fn dataset_provenance(
        &self,
    ) -> DatasetTrainingProvenance {
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

    pub fn auto_improvement_status(&self) -> &AutoImprovementStatus {
        &self.auto_improvement_status
    }

    pub fn model_registry(&self) -> &ModelRegistry {
        &self.model_registry
    }

    pub fn trainer_trigger_events_pending(&self) -> usize {
        self.trainer_trigger_events.len()
    }

    pub fn trainer_trigger_events(&self) -> &[TrainerTriggerEvent] {
        &self.trainer_trigger_events
    }

    pub fn pop_next_trainer_trigger_event(&mut self) -> Option<TrainerTriggerEvent> {
        if self.trainer_trigger_events.is_empty() {
            None
        } else {
            self.trainer_trigger_events.drain(0..1).next()
        }
    }

    pub fn lease_next_trainer_trigger_event(
        &mut self,
        now_epoch_seconds: u64,
        min_retry_delay_seconds: u64,
    ) -> Option<TrainerTriggerEvent> {
        let mut leased_idx = None;
        for (idx, event) in self.trainer_trigger_events.iter().enumerate() {
            let eligible = event.last_attempted_at.is_none_or(|last| {
                now_epoch_seconds >= last.saturating_add(min_retry_delay_seconds)
            });
            if eligible {
                leased_idx = Some(idx);
                break;
            }
        }
        let idx = leased_idx?;
        let event = self.trainer_trigger_events.get_mut(idx)?;
        event.delivery_attempts = event.delivery_attempts.saturating_add(1);
        event.last_attempted_at = Some(now_epoch_seconds);
        self.auto_improvement_status
            .trainer_trigger_delivery_attempts_total = self
            .auto_improvement_status
            .trainer_trigger_delivery_attempts_total
            .saturating_add(1);
        Some(event.clone())
    }

    pub fn acknowledge_trainer_trigger_event(&mut self, event_id: u64) -> bool {
        if let Some(idx) = self
            .trainer_trigger_events
            .iter()
            .position(|event| event.event_id == event_id)
        {
            self.trainer_trigger_events.remove(idx);
            self.auto_improvement_status
                .trainer_trigger_acknowledged_total = self
                .auto_improvement_status
                .trainer_trigger_acknowledged_total
                .saturating_add(1);
            true
        } else {
            false
        }
    }

    pub fn mark_trainer_trigger_event_delivery_failed(
        &mut self,
        event_id: u64,
        failed_at_epoch_seconds: u64,
    ) -> bool {
        if let Some(event) = self
            .trainer_trigger_events
            .iter_mut()
            .find(|event| event.event_id == event_id)
        {
            event.last_attempted_at = Some(failed_at_epoch_seconds);
            self.auto_improvement_status
                .trainer_trigger_delivery_failures_total = self
                .auto_improvement_status
                .trainer_trigger_delivery_failures_total
                .saturating_add(1);
            true
        } else {
            false
        }
    }

    pub fn drain_trainer_trigger_events(&mut self, max_events: usize) -> Vec<TrainerTriggerEvent> {
        if max_events == 0 || self.trainer_trigger_events.is_empty() {
            return Vec::new();
        }
        let drain_len = max_events.min(self.trainer_trigger_events.len());
        self.trainer_trigger_events.drain(0..drain_len).collect()
    }

    pub(in crate::orchestrator::pipeline_moe) fn push_trainer_trigger_event(
        &mut self,
        event: TrainerTriggerEvent,
    ) {
        self.trainer_trigger_events.push(event);
        if self.trainer_trigger_events.len() > self.max_trainer_trigger_events {
            let to_trim = self.trainer_trigger_events.len() - self.max_trainer_trigger_events;
            self.trainer_trigger_events.drain(0..to_trim);
        }
    }

    pub fn bootstrap_initial_dataset_from_training_bundle(
        &mut self,
        bundle: &DatasetTrainingBundle,
    ) -> Result<usize, MoeError> {
        let mut candidate = bundle.clone();
        candidate.ensure_checksum();
        DatasetStore::validate_training_bundle(&candidate)?;

        let mut seeded = 0usize;
        for sample in candidate
            .train_samples
            .iter()
            .chain(candidate.validation_samples.iter())
        {
            let id = format!("bootstrap:{}", sample.entry_id);
            let was_existing = self.dataset_store.has_entry_id(&id);

            self.dataset_store
                .upsert_entry(crate::dataset_engine::DatasetEntry {
                    id,
                    task_id: crate::moe_core::TaskId::new(&sample.task_id),
                    expert_id: crate::moe_core::ExpertId::new(&sample.expert_id),
                    input: sample.input.clone(),
                    output: sample.target_output.clone(),
                    outcome: crate::dataset_engine::Outcome::Success,
                    score: sample.score,
                    tags: {
                        let mut tags = sample.tags.clone();
                        if !tags.iter().any(|tag| tag == "bootstrap") {
                            tags.push("bootstrap".to_string());
                        }
                        tags
                    },
                    created_at: candidate.generated_at,
                    metadata: sample.metadata.clone(),
                });
            if !was_existing {
                seeded += 1;
            }
        }

        self.auto_improvement_status.bootstrap_entries_total += seeded;
        self.record_governance_audit("initial dataset bootstrap applied");
        Ok(seeded)
    }

    pub fn bootstrap_initial_dataset_from_training_bundle_json(
        &mut self,
        payload: &str,
    ) -> Result<usize, MoeError> {
        let bundle = self.preview_training_dataset_bundle_json(payload)?;
        self.bootstrap_initial_dataset_from_training_bundle(&bundle)
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
