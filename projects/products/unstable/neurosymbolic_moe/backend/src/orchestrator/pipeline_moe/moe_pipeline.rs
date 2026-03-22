//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/moe_pipeline.rs
use protocol::ProtocolId;

use crate::aggregator::OutputAggregator;
use crate::buffer_manager::BufferManager;
use crate::dataset_engine::{
    self, DatasetStore, DatasetTrainingBuildOptions, DatasetTrainingBundle,
    DatasetTrainingProvenance, DatasetTrainingShard,
};
use crate::evaluations::EvaluationEngine;
use crate::expert_registries::ExpertRegistry;
use crate::feedback_engine::{FeedbackEntry, FeedbackStore};
use crate::memory_engine::{LongTermMemory, MemoryEntry, MemoryStore, ShortTermMemory};
use crate::moe_core::{Expert, MoeError};
use crate::orchestrator::import_journal::ImportJournal;
use crate::orchestrator::{
    ArbitrationMode, AutoImprovementStatus, GovernanceState, ImportTelemetry, ModelRegistry,
    RuntimeImportReport, TrainerTriggerEvent, runtime_persistence_bundle,
};
use crate::orchestrator::{ContinuousImprovementReport, RuntimePersistenceBundle};
use crate::policies_guard::{Policy, PolicyGuard};
use crate::retrieval_engine::{ContextAssembler, Retriever};
use crate::router::Router;
use crate::trace_logging::TraceLogger;
use std::collections::VecDeque;

use super::{GovernanceRuntimeState, TrainerTriggerQueueState, TrainingRuntimeState};

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
const DEFAULT_TRAINER_TRIGGER_MIN_RETRY_DELAY_SECONDS: u64 = 30;
const DEFAULT_TRAINER_TRIGGER_MAX_DELIVERY_ATTEMPTS_BEFORE_DEAD_LETTER: u32 = 8;

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
    pub(in crate::orchestrator) governance_runtime_state: GovernanceRuntimeState,
    pub(in crate::orchestrator) policy_guard: PolicyGuard,
    pub(in crate::orchestrator) trace_logger: TraceLogger,
    pub(in crate::orchestrator) evaluation: EvaluationEngine,
    pub(in crate::orchestrator) import_telemetry: ImportTelemetry,
    pub(in crate::orchestrator) last_runtime_import_report: Option<RuntimeImportReport>,
    pub(in crate::orchestrator) import_journal: ImportJournal,
    pub(in crate::orchestrator) training_runtime_state: TrainingRuntimeState,
    pub(in crate::orchestrator) trainer_trigger_queue: TrainerTriggerQueueState,
}

impl MoePipeline {
    pub fn validate_runtime_invariants(&self) -> Result<(), MoeError> {
        self.governance_runtime_state.validate_invariants()?;
        self.training_runtime_state.validate_invariants()?;
        self.trainer_trigger_queue.validate_invariants()?;
        Ok(())
    }

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
        session_id: &ProtocolId,
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
        &self.training_runtime_state.feedback_store
    }

    pub fn dataset_store(&self) -> &DatasetStore {
        &self.training_runtime_state.dataset_store
    }

    pub fn import_telemetry_snapshot(&self) -> ImportTelemetry {
        self.import_telemetry.clone()
    }

    pub fn last_runtime_import_report(&self) -> Option<&RuntimeImportReport> {
        self.last_runtime_import_report.as_ref()
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
        self.training_runtime_state
            .dataset_store
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

    pub fn parse_training_dataset_shards_json(
        &self,
        payload: &str,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        self.rebuild_training_dataset_bundle_from_shards_json(payload)
    }

    pub fn preview_training_dataset_shards_json(
        &self,
        payload: &str,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        self.parse_training_dataset_shards_json(payload)
    }

    pub(crate) fn runtime_bundle_checksum(&self) -> String {
        runtime_persistence_bundle::recompute_runtime_checksum_from_components(
            RuntimePersistenceBundle::schema_version(),
            GovernanceState::schema_version(),
            self.governance_runtime_state
                .governance_state_version
                .clone(),
            &self.governance_state_checksum(),
            &self.governance_runtime_state.governance_audit_entries,
            &self.governance_runtime_state.governance_state_snapshots,
            self.short_term_memory.entries(),
            self.long_term_memory.entries(),
            &self.buffer_manager,
            self.training_runtime_state.dataset_store.entries(),
            self.training_runtime_state.dataset_store.corrections(),
            self.training_runtime_state.auto_improvement_policy.as_ref(),
            &self.training_runtime_state.auto_improvement_status,
            &self.training_runtime_state.model_registry,
            self.trainer_trigger_queue.events().iter(),
            self.trainer_trigger_queue.dead_letter_events().iter(),
            self.trainer_trigger_queue
                .leased_event_ids_sorted()
                .iter()
                .collect::<Vec<_>>(),
        )
    }

    fn governance_state_checksum(&self) -> String {
        GovernanceState::recompute_checksum_from_components(
            GovernanceState::schema_version(),
            &self.governance_runtime_state.governance_state_version,
            self.governance_runtime_state
                .continuous_governance_policy
                .as_ref(),
            self.governance_runtime_state.evaluation_baseline.as_ref(),
            self.governance_runtime_state
                .last_continuous_improvement_report
                .as_ref(),
        )
    }

    pub(in crate::orchestrator::pipeline_moe) fn dataset_provenance(
        &self,
    ) -> DatasetTrainingProvenance {
        DatasetTrainingProvenance {
            generator: "neurosymbolic_moe_backend.orchestrator".to_string(),
            governance_state_version: self
                .governance_runtime_state
                .governance_state_version
                .clone(),
            governance_state_checksum: self.governance_state_checksum(),
            runtime_bundle_checksum: self.runtime_bundle_checksum(),
            dataset_entry_count: self.training_runtime_state.dataset_store.count(),
        }
    }

    pub fn add_feedback(&mut self, entry: FeedbackEntry) {
        self.training_runtime_state.feedback_store.add(entry);
    }

    pub fn auto_improvement_status(&self) -> &AutoImprovementStatus {
        &self.training_runtime_state.auto_improvement_status
    }

    pub fn model_registry(&self) -> &ModelRegistry {
        &self.training_runtime_state.model_registry
    }

    pub fn trainer_trigger_events_pending(&self) -> usize {
        self.trainer_trigger_queue.len()
    }

    pub fn trainer_trigger_events(&self) -> &VecDeque<TrainerTriggerEvent> {
        self.trainer_trigger_queue.events()
    }

    pub fn trainer_trigger_dead_letter_events(&self) -> &VecDeque<TrainerTriggerEvent> {
        self.trainer_trigger_queue.dead_letter_events()
    }

    pub fn trainer_trigger_dead_letter_events_total(&self) -> usize {
        self.trainer_trigger_queue.dead_letter_count()
    }

    pub fn pop_next_trainer_trigger_event(&mut self) -> Option<TrainerTriggerEvent> {
        self.trainer_trigger_queue.pop_next()
    }

    pub fn lease_next_trainer_trigger_event_with_policy(
        &mut self,
        now_epoch_seconds: u64,
    ) -> Option<TrainerTriggerEvent> {
        self.lease_next_trainer_trigger_event(
            now_epoch_seconds,
            self.trainer_trigger_min_retry_delay_seconds(),
        )
    }

    pub fn lease_next_trainer_trigger_event(
        &mut self,
        now_epoch_seconds: u64,
        min_retry_delay_seconds: u64,
    ) -> Option<TrainerTriggerEvent> {
        let dead_letter_before = self.trainer_trigger_queue.dead_letter_count();
        let leased = self.trainer_trigger_queue.lease_next(
            now_epoch_seconds,
            min_retry_delay_seconds,
            self.trainer_trigger_max_delivery_attempts_before_dead_letter(),
        );
        let dead_letter_after = self.trainer_trigger_queue.dead_letter_count();
        let new_dead_letters = dead_letter_after.saturating_sub(dead_letter_before) as u64;
        if new_dead_letters > 0 {
            self.training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .dead_letter_total = self
                .training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .dead_letter_total
                .saturating_add(new_dead_letters);
        }
        if leased.is_some() {
            self.training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .delivery_attempts_total = self
                .training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .delivery_attempts_total
                .saturating_add(1);
        }
        leased
    }

    #[allow(dead_code)]
    pub fn acknowledge_trainer_trigger_event(&mut self, event_id: ProtocolId) -> bool {
        if self.trainer_trigger_queue.acknowledge(&event_id) {
            self.training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .acknowledged_total = self
                .training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .acknowledged_total
                .saturating_add(1);
            true
        } else {
            false
        }
    }

    pub fn mark_trainer_trigger_event_delivery_failed(
        &mut self,
        event_id: ProtocolId,
        failed_at_epoch_seconds: u64,
    ) -> bool {
        if self
            .trainer_trigger_queue
            .mark_delivery_failed(&event_id, failed_at_epoch_seconds)
        {
            self.training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .delivery_failures_total = self
                .training_runtime_state
                .auto_improvement_status
                .delivery_stats
                .delivery_failures_total
                .saturating_add(1);
            true
        } else {
            false
        }
    }

    pub(in crate::orchestrator::pipeline_moe) fn push_trainer_trigger_event(
        &mut self,
        event: TrainerTriggerEvent,
    ) {
        self.trainer_trigger_queue.push(event);
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
            let id = sample.entry_id;
            let was_existing = self.training_runtime_state.dataset_store.has_entry_id(&id);

            self.training_runtime_state
                .dataset_store
                .upsert_entry(dataset_engine::DatasetEntry {
                    id,
                    task_id: sample.task_id.clone(),
                    expert_id: sample.expert_id.clone(),
                    input: sample.input.clone(),
                    output: sample.target_output.clone(),
                    outcome: dataset_engine::Outcome::Success,
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

        self.training_runtime_state
            .auto_improvement_status
            .global_counters
            .bootstrap_entries_total = self
            .training_runtime_state
            .auto_improvement_status
            .global_counters
            .bootstrap_entries_total
            .saturating_add(seeded);
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
        self.governance_runtime_state.evaluation_baseline = Some(self.evaluation.clone());
    }

    pub fn last_continuous_improvement_report(&self) -> Option<&ContinuousImprovementReport> {
        self.governance_runtime_state
            .last_continuous_improvement_report
            .as_ref()
    }

    pub fn has_evaluation_baseline(&self) -> bool {
        self.governance_runtime_state.evaluation_baseline.is_some()
    }

    pub fn approve_pending_human_review_and_promote(&mut self) -> bool {
        if self
            .governance_runtime_state
            .last_continuous_improvement_report
            .as_ref()
            .is_some_and(|report| report.requires_human_review)
        {
            self.capture_evaluation_baseline();
            if let Some(report) = self
                .governance_runtime_state
                .last_continuous_improvement_report
                .as_mut()
            {
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
            self.governance_runtime_state
                .governance_state_version
                .clone(),
            self.governance_runtime_state
                .continuous_governance_policy
                .clone(),
            self.governance_runtime_state.evaluation_baseline.clone(),
            self.governance_runtime_state
                .last_continuous_improvement_report
                .clone(),
        )
    }

    pub(in crate::orchestrator::pipeline_moe) fn trainer_trigger_min_retry_delay_seconds(
        &self,
    ) -> u64 {
        self.training_runtime_state
            .auto_improvement_policy
            .as_ref()
            .map(|policy| policy.trainer_trigger_min_retry_delay_seconds)
            .unwrap_or(DEFAULT_TRAINER_TRIGGER_MIN_RETRY_DELAY_SECONDS)
    }

    fn trainer_trigger_max_delivery_attempts_before_dead_letter(&self) -> u32 {
        self.training_runtime_state
            .auto_improvement_policy
            .as_ref()
            .map(|policy| policy.trainer_trigger_max_delivery_attempts_before_dead_letter)
            .unwrap_or(DEFAULT_TRAINER_TRIGGER_MAX_DELIVERY_ATTEMPTS_BEFORE_DEAD_LETTER)
            .max(1)
    }
}
