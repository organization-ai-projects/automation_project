//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/moe_pipeline.rs
use crate::aggregator::OutputAggregator;
use crate::buffer_manager::BufferManager;
use crate::dataset_engine::{DatasetStore, Outcome, TraceConverter};
use crate::evaluation_engine::EvaluationEngine;
use crate::expert_registry::ExpertRegistry;
use crate::feedback_engine::{FeedbackEntry, FeedbackStore};
use crate::memory_engine::{
    LongTermMemory, MemoryEntry, MemoryQuery, MemoryStore, ShortTermMemory,
};
use crate::moe_core::{
    AggregatedOutput, ExecutionContext, Expert, ExpertError, ExpertId, ExpertOutput, MoeError,
    Task, TracePhase,
};
use crate::orchestrator::ContinuousImprovementReport;
use crate::orchestrator::{
    ArbitrationMode, ContinuousGovernancePolicy, GovernanceAuditEntry, GovernanceAuditTrail,
    GovernanceImportDecision, GovernanceImportPolicy, GovernancePersistenceBundle, GovernanceState,
    GovernanceStateDiff, GovernanceStateSnapshot, RuntimePersistenceBundle,
};
use crate::policy_guard::{Policy, PolicyGuard};
use crate::retrieval_engine::{ContextAssembler, RetrievalQuery, Retriever};
use crate::router::{Router, RoutingStrategy};
use crate::trace_logger::TraceLogger;

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

    pub fn execute(&mut self, task: Task) -> Result<AggregatedOutput, MoeError> {
        let task_id = task.id().clone();
        let chain_ids = if self.enable_task_metadata_chain {
            task.metadata
                .get("expert_chain")
                .map(|raw| Self::parse_expert_chain(raw))
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let chain_enabled = !chain_ids.is_empty();

        // 1. Routing phase
        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::Routing,
            "routing phase begins".to_string(),
            None,
        );

        let (selected_experts, routing_scores, decision_strategy, routing_explanation) =
            if chain_enabled {
                self.trace_logger.log_phase(
                    task_id.clone(),
                    TracePhase::Routing,
                    "routing bypassed by task metadata expert_chain".to_string(),
                    None,
                );
                (
                    chain_ids,
                    std::collections::HashMap::new(),
                    RoutingStrategy::MultiExpert,
                    "expert chain from task metadata".to_string(),
                )
            } else {
                let decision = self.router.route(&task, &self.registry).map_err(|e| {
                    self.trace_logger.log_phase(
                        task_id.clone(),
                        TracePhase::Routing,
                        format!("routing failed: {e}"),
                        None,
                    );
                    e
                })?;
                (
                    decision.selected_experts,
                    decision.scores,
                    decision.strategy,
                    decision.explanation,
                )
            };

        let mut used_fallback = matches!(decision_strategy, RoutingStrategy::Fallback);

        // 2. Expert selection
        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::ExpertSelection,
            format!(
                "selected {} expert(s): {}",
                selected_experts.len(),
                routing_explanation
            ),
            None,
        );

        // 3. Build execution context using retrieval + memory + buffer context.
        let context = self.build_execution_context(&task, &selected_experts)?;

        // 4. Execute selected experts
        let mut outputs: Vec<ExpertOutput> = Vec::new();
        let mut chain_input: Option<String> = None;

        for (index, expert_id) in selected_experts.iter().enumerate() {
            let expert = self.registry.get(expert_id).ok_or_else(|| {
                let msg = format!("expert '{}' not found in registry", expert_id.as_str());
                self.trace_logger.log_phase(
                    task_id.clone(),
                    TracePhase::ExpertExecution,
                    msg.clone(),
                    Some(expert_id.clone()),
                );
                MoeError::NoExpertFound(msg)
            })?;

            let mut expert_task = task.clone();
            if chain_enabled && index > 0 {
                expert_task.input = chain_input.clone().unwrap_or_else(|| task.input.clone());
            }

            match expert.execute(&expert_task, &context) {
                Ok(output) => {
                    if chain_enabled
                        && index + 1 < selected_experts.len()
                        && self.policy_guard.active_policy_count() > 0
                        && let Err(err) = self.policy_guard.validate_strict(&output)
                    {
                        self.trace_logger.log_phase(
                            task_id.clone(),
                            TracePhase::Validation,
                            format!(
                                "chained output from '{}' rejected by policy before propagation: {err}",
                                expert_id.as_str()
                            ),
                            Some(expert_id.clone()),
                        );
                        return Err(err);
                    }

                    self.trace_logger.log_phase(
                        task_id.clone(),
                        TracePhase::ExpertExecution,
                        format!(
                            "expert '{}' executed successfully (confidence: {:.2})",
                            expert_id.as_str(),
                            output.confidence
                        ),
                        Some(expert_id.clone()),
                    );
                    self.evaluation.record_expert_execution(
                        expert_id.clone(),
                        true,
                        output.confidence,
                        0.0,
                    );
                    if chain_enabled {
                        chain_input = Some(output.content.clone());
                    }
                    outputs.push(output);
                }
                Err(e) => {
                    self.trace_logger.log_phase(
                        task_id.clone(),
                        TracePhase::ExpertExecution,
                        format!("expert '{}' failed: {e}", expert_id.as_str()),
                        Some(expert_id.clone()),
                    );
                    self.evaluation
                        .record_expert_execution(expert_id.clone(), false, 0.0, 0.0);
                    if self.fallback_on_expert_error {
                        used_fallback = true;
                        continue;
                    }
                    return Err(MoeError::ExpertError(e));
                }
            }
        }

        if outputs.is_empty() {
            return Err(MoeError::ExpertError(ExpertError::ExecutionFailed(
                "all selected experts failed to produce outputs".to_string(),
            )));
        }

        self.evaluation
            .record_routing(selected_experts.len(), used_fallback);

        // 5. Aggregation
        let mut aggregated = self.aggregator.aggregate(outputs).map_err(|e| {
            self.trace_logger.log_phase(
                task_id.clone(),
                TracePhase::Aggregation,
                format!("aggregation failed: {e}"),
                None,
            );
            e
        })?;

        if matches!(self.arbitration_mode, ArbitrationMode::RouterScoreWeighted) {
            let selected = aggregated
                .outputs
                .iter()
                .max_by(|a, b| {
                    let score_a = self.arbitration_score(a, &routing_scores);
                    let score_b = self.arbitration_score(b, &routing_scores);
                    score_a
                        .partial_cmp(&score_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned();
            aggregated.selected_output = selected;
            aggregated.strategy = format!("router_score_weighted+{}", aggregated.strategy);
        }

        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::Aggregation,
            format!("aggregated with strategy '{}'", aggregated.strategy),
            None,
        );

        if used_fallback {
            self.trace_logger.log_phase(
                task_id.clone(),
                TracePhase::Aggregation,
                "fallback path used during expert execution".to_string(),
                None,
            );
        }

        if aggregated.selected_output.is_none() {
            self.trace_logger.log_phase(
                task_id.clone(),
                TracePhase::Validation,
                "policy validation failed: aggregated output has no selected output".to_string(),
                None,
            );
            return Err(MoeError::PolicyRejected(
                "aggregated output has no selected output".to_string(),
            ));
        }

        // 6. Policy validation
        if let Some(ref selected) = aggregated.selected_output
            && let Err(err) = self.policy_guard.validate_strict(selected)
        {
            if self.policy_guard.active_policy_count() > 0 {
                let replacement = self.pick_policy_compliant_output(
                    &aggregated.outputs,
                    &routing_scores,
                    Some(&selected.expert_id),
                );
                if let Some(replacement) = replacement {
                    self.trace_logger.log_phase(
                        task_id.clone(),
                        TracePhase::Validation,
                        format!(
                            "selected output from '{}' rejected, replaced by policy-compliant '{}'",
                            selected.expert_id.as_str(),
                            replacement.expert_id.as_str()
                        ),
                        None,
                    );
                    aggregated.selected_output = Some(replacement);
                    aggregated.strategy = format!("{}+policy_fallback", aggregated.strategy);
                } else {
                    self.trace_logger.log_phase(
                        task_id.clone(),
                        TracePhase::Validation,
                        format!("policy validation failed: {err}"),
                        None,
                    );
                    return Err(err);
                }
            } else {
                self.trace_logger.log_phase(
                    task_id.clone(),
                    TracePhase::Validation,
                    format!("policy validation failed: {err}"),
                    None,
                );
                return Err(err);
            }
        }

        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::Validation,
            "policy validation passed".to_string(),
            None,
        );

        // 7. Convert traces to dataset entry
        let task_traces: Vec<_> = self
            .trace_logger
            .get_by_task(&task_id)
            .into_iter()
            .cloned()
            .collect();

        let output_text = aggregated
            .selected_output
            .as_ref()
            .map(|o| o.content.as_str())
            .unwrap_or("");

        let entry =
            self.trace_converter
                .convert(&task_traces, task.input(), output_text, Outcome::Success);

        self.dataset_store.add_entry(entry);

        self.trace_logger.log_phase(
            task_id,
            TracePhase::DatasetEnrichment,
            "traces converted to dataset entry".to_string(),
            None,
        );

        if let Some(policy) = &self.continuous_governance_policy {
            let report = self.continuous_improvement_report(
                policy.min_expert_success_rate,
                policy.min_routing_accuracy,
                policy.low_score_threshold,
                policy.regression_drop_threshold,
            );

            if report.requires_human_review {
                self.trace_logger.log_phase(
                    task.id().clone(),
                    TracePhase::Validation,
                    "continuous governance review required".to_string(),
                    None,
                );

                if policy.block_on_human_review {
                    self.last_continuous_improvement_report = Some(report);
                    self.record_governance_audit("continuous governance blocked pending review");
                    return Err(MoeError::PolicyRejected(
                        "continuous governance gate blocked output (human review required)"
                            .to_string(),
                    ));
                }
            } else {
                self.trace_logger.log_phase(
                    task.id().clone(),
                    TracePhase::Validation,
                    "continuous governance gate passed".to_string(),
                    None,
                );

                if policy.auto_promote_on_pass {
                    self.capture_evaluation_baseline();
                    self.record_governance_audit("continuous governance auto-promotion");
                    self.trace_logger.log_phase(
                        task.id().clone(),
                        TracePhase::Validation,
                        "continuous governance auto-promotion captured new baseline".to_string(),
                        None,
                    );
                }
            }

            self.last_continuous_improvement_report = Some(report);
            self.record_governance_audit("continuous governance evaluated");
        }

        Ok(aggregated)
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

    pub fn import_governance_state(&mut self, mut state: GovernanceState) {
        state.ensure_checksum();
        if !state.verify_checksum() {
            self.trace_logger.log_phase(
                crate::moe_core::TaskId::new("governance-import"),
                TracePhase::Validation,
                "governance state checksum mismatch during import".to_string(),
                None,
            );
            return;
        }

        let decision = self.evaluate_governance_import(&state);
        if !decision.allowed {
            self.trace_logger.log_phase(
                crate::moe_core::TaskId::new("governance-import"),
                TracePhase::Validation,
                format!(
                    "governance import rejected: {}",
                    decision.reasons.join("; ")
                ),
                None,
            );
            return;
        }

        self.continuous_governance_policy = state.continuous_governance_policy;
        self.evaluation_baseline = state.evaluation_baseline;
        self.last_continuous_improvement_report = state.last_continuous_improvement_report;
        self.governance_state_version = state.state_version;
        self.record_governance_audit("governance state imported");
    }

    pub fn export_governance_state_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_governance_state()).map_err(|err| {
            MoeError::DatasetError(format!("governance state serialization failed: {err}"))
        })
    }

    pub fn export_governance_bundle(&self) -> GovernancePersistenceBundle {
        GovernancePersistenceBundle {
            state: self.export_governance_state(),
            audit_entries: self.governance_audit_entries.clone(),
            snapshots: self.governance_state_snapshots.clone(),
        }
    }

    pub fn export_governance_bundle_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_governance_bundle()).map_err(|err| {
            MoeError::DatasetError(format!(
                "governance persistence bundle serialization failed: {err}"
            ))
        })
    }

    pub fn export_runtime_bundle(&self) -> RuntimePersistenceBundle {
        RuntimePersistenceBundle::from_components(
            self.export_governance_bundle(),
            self.short_term_memory.entries_cloned(),
            self.long_term_memory.entries_cloned(),
            self.buffer_manager.clone(),
        )
    }

    pub fn export_runtime_bundle_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_runtime_bundle()).map_err(|err| {
            MoeError::DatasetError(format!(
                "runtime persistence bundle serialization failed: {err}"
            ))
        })
    }

    pub fn import_governance_state_json(&mut self, payload: &str) -> Result<(), MoeError> {
        self.try_import_governance_state_json(payload)
    }

    pub fn import_governance_bundle(
        &mut self,
        bundle: GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_governance_bundle_import(&bundle)?;
        if !decision.allowed {
            return Err(MoeError::PolicyRejected(format!(
                "governance bundle rejected: {}",
                decision.reasons.join("; ")
            )));
        }

        self.continuous_governance_policy = bundle.state.continuous_governance_policy.clone();
        self.evaluation_baseline = bundle.state.evaluation_baseline.clone();
        self.last_continuous_improvement_report =
            bundle.state.last_continuous_improvement_report.clone();
        self.governance_state_version = bundle.state.state_version;

        self.governance_audit_entries = bundle.audit_entries;
        if self.governance_audit_entries.len() > self.max_governance_audit_entries {
            let to_trim = self.governance_audit_entries.len() - self.max_governance_audit_entries;
            self.governance_audit_entries.drain(0..to_trim);
        }

        self.governance_state_snapshots = bundle.snapshots;
        if self.governance_state_snapshots.len() > self.max_governance_state_snapshots {
            let to_trim =
                self.governance_state_snapshots.len() - self.max_governance_state_snapshots;
            self.governance_state_snapshots.drain(0..to_trim);
        }
        self.retain_snapshots_with_matching_audit_versions();

        Ok(())
    }

    pub fn import_governance_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let bundle: GovernancePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "governance persistence bundle deserialization failed: {err}"
                ))
            })?;
        self.import_governance_bundle(bundle)
    }

    pub fn import_runtime_bundle(
        &mut self,
        bundle: RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_runtime_bundle_import(&bundle)?;
        if !decision.allowed {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: {}",
                decision.reasons.join("; ")
            )));
        }
        self.import_governance_bundle(bundle.governance)?;
        self.short_term_memory
            .replace_entries(bundle.short_term_memory_entries)?;
        self.long_term_memory
            .replace_entries(bundle.long_term_memory_entries)?;
        self.buffer_manager = bundle.buffer_manager;
        Ok(())
    }

    pub fn import_runtime_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let mut bundle: RuntimePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "runtime persistence bundle deserialization failed: {err}"
                ))
            })?;
        bundle.ensure_checksum();
        self.import_runtime_bundle(bundle)
    }

    pub fn try_import_runtime_bundle(
        &mut self,
        bundle: RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_runtime_bundle_import(&bundle)?;
        if !decision.allowed {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle import rejected: {}",
                decision.reasons.join("; ")
            )));
        }
        self.import_runtime_bundle(bundle)
    }

    pub fn try_import_runtime_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let mut bundle: RuntimePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "runtime persistence bundle deserialization failed: {err}"
                ))
            })?;
        bundle.ensure_checksum();
        self.try_import_runtime_bundle(bundle)
    }

    pub fn preview_runtime_bundle_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        let mut bundle: RuntimePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "runtime persistence bundle deserialization failed: {err}"
                ))
            })?;
        bundle.ensure_checksum();
        self.evaluate_runtime_bundle_import(&bundle)
    }

    pub fn try_import_governance_bundle(
        &mut self,
        bundle: GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_governance_bundle_import(&bundle)?;
        if !decision.allowed {
            return Err(MoeError::PolicyRejected(format!(
                "governance bundle import rejected: {}",
                decision.reasons.join("; ")
            )));
        }

        self.import_governance_bundle(bundle)
    }

    pub fn try_import_governance_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let bundle: GovernancePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "governance persistence bundle deserialization failed: {err}"
                ))
            })?;
        self.try_import_governance_bundle(bundle)
    }

    pub fn try_import_governance_state(
        &mut self,
        mut state: GovernanceState,
    ) -> Result<(), MoeError> {
        state.ensure_checksum();
        if !state.verify_checksum() {
            return Err(MoeError::PolicyRejected(
                "governance state checksum verification failed".to_string(),
            ));
        }

        let decision = self.evaluate_governance_import(&state);
        if !decision.allowed {
            return Err(MoeError::PolicyRejected(format!(
                "governance import rejected: {}",
                decision.reasons.join("; ")
            )));
        }

        self.import_governance_state(state);
        Ok(())
    }

    pub fn preview_governance_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        let mut state: GovernanceState =
            common_json::json::from_json_str(payload).map_err(|err| {
                MoeError::DatasetError(format!("governance state deserialization failed: {err}"))
            })?;
        state.ensure_checksum();
        if !state.verify_checksum() {
            return Err(MoeError::PolicyRejected(
                "governance state checksum verification failed".to_string(),
            ));
        }
        Ok(self.evaluate_governance_import(&state))
    }

    pub fn preview_governance_bundle_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        let bundle: GovernancePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "governance persistence bundle deserialization failed: {err}"
                ))
            })?;
        self.evaluate_governance_bundle_import(&bundle)
    }

    pub fn try_import_governance_state_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let mut state: GovernanceState =
            common_json::json::from_json_str(payload).map_err(|err| {
                MoeError::DatasetError(format!("governance state deserialization failed: {err}"))
            })?;
        state.ensure_checksum();
        self.try_import_governance_state(state)
    }

    pub fn governance_audit_trail(&self) -> GovernanceAuditTrail {
        GovernanceAuditTrail {
            current_version: self.governance_state_version,
            current_checksum: self
                .governance_audit_entries
                .last()
                .map(|e| e.checksum.clone()),
            entries: self.governance_audit_entries.clone(),
        }
    }

    pub fn governance_state_snapshots(&self) -> &[GovernanceStateSnapshot] {
        &self.governance_state_snapshots
    }

    pub fn rollback_governance_state_to_version(&mut self, version: u64) -> Result<(), MoeError> {
        let snapshot = self
            .governance_state_snapshots
            .iter()
            .rev()
            .find(|snapshot| snapshot.version == version)
            .cloned()
            .ok_or_else(|| {
                MoeError::DatasetError(format!(
                    "governance rollback failed: snapshot version {} not found",
                    version
                ))
            })?;

        self.continuous_governance_policy = snapshot.state.continuous_governance_policy;
        self.evaluation_baseline = snapshot.state.evaluation_baseline;
        self.last_continuous_improvement_report = snapshot.state.last_continuous_improvement_report;
        self.governance_state_version = snapshot.state.state_version;
        self.record_governance_audit(&format!("governance rollback to version {}", version));
        Ok(())
    }

    pub fn diff_governance_state(&self, target: &GovernanceState) -> GovernanceStateDiff {
        let source = self.export_governance_state();

        let source_version = source.state_version;
        let target_version = target.state_version;
        let version_delta = target_version as i64 - source_version as i64;

        let source_policy_fp = source
            .continuous_governance_policy
            .as_ref()
            .map(|p| {
                format!(
                    "{:.6}:{:.6}:{:.6}:{:.6}:{}:{}",
                    p.min_expert_success_rate,
                    p.min_routing_accuracy,
                    p.low_score_threshold,
                    p.regression_drop_threshold,
                    p.block_on_human_review,
                    p.auto_promote_on_pass
                )
            })
            .unwrap_or_else(|| "-".to_string());
        let target_policy_fp = target
            .continuous_governance_policy
            .as_ref()
            .map(|p| {
                format!(
                    "{:.6}:{:.6}:{:.6}:{:.6}:{}:{}",
                    p.min_expert_success_rate,
                    p.min_routing_accuracy,
                    p.low_score_threshold,
                    p.regression_drop_threshold,
                    p.block_on_human_review,
                    p.auto_promote_on_pass
                )
            })
            .unwrap_or_else(|| "-".to_string());

        let source_baseline_fp = source
            .evaluation_baseline
            .as_ref()
            .map(EvaluationEngine::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());
        let target_baseline_fp = target
            .evaluation_baseline
            .as_ref()
            .map(EvaluationEngine::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());

        let source_report_fp = source
            .last_continuous_improvement_report
            .as_ref()
            .map(ContinuousImprovementReport::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());
        let target_report_fp = target
            .last_continuous_improvement_report
            .as_ref()
            .map(ContinuousImprovementReport::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());

        let schema_version_changed = source.schema_version != target.schema_version;
        let checksum_changed = source.state_checksum != target.state_checksum;
        let policy_changed = source_policy_fp != target_policy_fp;
        let baseline_changed = source_baseline_fp != target_baseline_fp;
        let report_changed = source_report_fp != target_report_fp;

        let has_drift = schema_version_changed
            || checksum_changed
            || policy_changed
            || baseline_changed
            || report_changed
            || version_delta != 0;

        GovernanceStateDiff {
            source_version,
            target_version,
            version_delta,
            schema_version_changed,
            checksum_changed,
            policy_changed,
            baseline_changed,
            report_changed,
            has_drift,
        }
    }

    pub fn continuous_improvement_report(
        &self,
        min_expert_success_rate: f64,
        min_routing_accuracy: f64,
        low_score_threshold: f64,
        regression_drop_threshold: f64,
    ) -> ContinuousImprovementReport {
        let governance = self
            .evaluation
            .governance_report(min_expert_success_rate, min_routing_accuracy);
        let dataset_quality = self.dataset_store.quality_report(low_score_threshold);

        let (expert_regressions, routing_regression) =
            if let Some(baseline) = &self.evaluation_baseline {
                (
                    self.evaluation
                        .detect_expert_regressions(baseline, regression_drop_threshold),
                    self.evaluation
                        .detect_routing_regression(baseline, regression_drop_threshold),
                )
            } else {
                (Vec::new(), None)
            };

        let requires_human_review = !governance.ready_for_promotion
            || !expert_regressions.is_empty()
            || routing_regression.is_some()
            || dataset_quality.low_score_entries > 0;

        ContinuousImprovementReport {
            governance,
            dataset_quality,
            expert_regressions,
            routing_regression,
            requires_human_review,
        }
    }

    fn build_execution_context(
        &mut self,
        task: &Task,
        selected_experts: &[ExpertId],
    ) -> Result<ExecutionContext, MoeError> {
        let retrieval_max_results =
            Self::metadata_usize(task, "retrieval.max_results").unwrap_or(8);
        let retrieval_min_relevance =
            Self::metadata_f64(task, "retrieval.min_relevance").unwrap_or(0.05);
        let mut query = RetrievalQuery::new(task.input())
            .with_task_id(task.id().clone())
            .with_max_results(retrieval_max_results)
            .with_min_relevance(retrieval_min_relevance);
        if let Some(expert_id) = selected_experts.first() {
            query = query.with_expert_id(expert_id.clone());
        }
        for (key, value) in &task.metadata {
            if let Some(filter_key) = key.strip_prefix("retrieval.filter.") {
                query = query.with_filter(filter_key, value);
            }
        }

        let retrieval_results = self
            .retriever
            .retrieve(&query)
            .map_err(|err| MoeError::RetrievalFailed(format!("context retrieval failed: {err}")))?;
        let retrieved_context = self
            .context_assembler
            .assemble_for_task(&retrieval_results, task);
        self.trace_logger.log_phase(
            task.id().clone(),
            TracePhase::Retrieval,
            format!(
                "retrieval context assembled (results={}, segments={})",
                retrieval_results.len(),
                retrieved_context.len()
            ),
            None,
        );

        let memory_tags = Self::metadata_csv(task, "memory.tags");
        let memory_query = MemoryQuery {
            tags: if memory_tags.is_empty() {
                None
            } else {
                Some(memory_tags)
            },
            memory_type: None,
            min_relevance: Some(Self::metadata_f64(task, "memory.min_relevance").unwrap_or(0.0)),
            max_results: Self::metadata_usize(task, "memory.max_results").unwrap_or(16),
            include_expired: Self::metadata_bool(task, "memory.include_expired").unwrap_or(false),
            current_time: None,
        };
        let mut memory_entries: Vec<String> = self
            .short_term_memory
            .retrieve(&memory_query)?
            .into_iter()
            .map(|entry| entry.content.clone())
            .collect();
        memory_entries.extend(
            self.long_term_memory
                .retrieve(&memory_query)?
                .into_iter()
                .map(|entry| entry.content.clone()),
        );
        memory_entries.sort_unstable();
        memory_entries.dedup();
        self.trace_logger.log_phase(
            task.id().clone(),
            TracePhase::MemoryQuery,
            format!(
                "memory context assembled (entries={})",
                memory_entries.len()
            ),
            None,
        );

        let working_key = format!("task/{}", task.id().as_str());
        self.buffer_manager.working_mut().put(
            working_key,
            task.input().to_string(),
            Some(task.id().clone()),
        );
        let mut buffer_keys = self.buffer_manager.working().keys();
        buffer_keys.sort_unstable();
        let mut buffer_data: Vec<String> = buffer_keys
            .into_iter()
            .filter_map(|key| {
                self.buffer_manager
                    .working()
                    .get(key)
                    .map(|entry| entry.value.clone())
            })
            .collect();
        if let Some(session_id) = task.metadata.get("session_id") {
            buffer_data.extend(self.buffer_manager.sessions().values(session_id));
        }

        Ok(ExecutionContext::new(task.id().clone())
            .with_retrieved_context(retrieved_context)
            .with_memory_entries(memory_entries)
            .with_buffer_data(buffer_data))
    }

    fn metadata_usize(task: &Task, key: &str) -> Option<usize> {
        task.metadata
            .get(key)
            .and_then(|value| value.parse::<usize>().ok())
    }

    fn metadata_f64(task: &Task, key: &str) -> Option<f64> {
        task.metadata
            .get(key)
            .and_then(|value| value.parse::<f64>().ok())
    }

    fn metadata_bool(task: &Task, key: &str) -> Option<bool> {
        task.metadata
            .get(key)
            .and_then(|value| value.parse::<bool>().ok())
    }

    fn metadata_csv(task: &Task, key: &str) -> Vec<String> {
        task.metadata
            .get(key)
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn parse_expert_chain(raw: &str) -> Vec<ExpertId> {
        raw.split([',', '>'])
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(ExpertId::new)
            .collect()
    }

    fn evaluate_governance_bundle_import(
        &self,
        bundle: &GovernancePersistenceBundle,
    ) -> Result<GovernanceImportDecision, MoeError> {
        let mut state = bundle.state.clone();
        state.ensure_checksum();
        if !state.verify_checksum() {
            return Err(MoeError::PolicyRejected(
                "governance bundle checksum verification failed".to_string(),
            ));
        }
        if !bundle
            .snapshots
            .iter()
            .all(|snapshot| snapshot.state.verify_checksum())
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle checksum verification failed".to_string(),
            ));
        }

        let mut normalized_bundle = bundle.clone();
        normalized_bundle.state = state;
        Self::validate_governance_bundle_consistency(&normalized_bundle)?;
        Ok(self.evaluate_governance_import(&normalized_bundle.state))
    }

    fn evaluate_runtime_bundle_import(
        &self,
        bundle: &RuntimePersistenceBundle,
    ) -> Result<GovernanceImportDecision, MoeError> {
        if !bundle.has_supported_schema() {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle schema version {} is not supported",
                bundle.schema_version
            )));
        }
        if !bundle.verify_checksum() {
            return Err(MoeError::PolicyRejected(
                "runtime bundle checksum verification failed".to_string(),
            ));
        }
        self.evaluate_governance_bundle_import(&bundle.governance)
    }

    fn validate_governance_bundle_consistency(
        bundle: &GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        if bundle
            .audit_entries
            .windows(2)
            .any(|pair| pair[0].version >= pair[1].version)
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: audit versions must be strictly increasing"
                    .to_string(),
            ));
        }

        if bundle
            .snapshots
            .windows(2)
            .any(|pair| pair[0].version >= pair[1].version)
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: snapshot versions must be strictly increasing"
                    .to_string(),
            ));
        }

        if let Some(last_audit) = bundle.audit_entries.last() {
            if last_audit.version != bundle.state.state_version {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest audit version does not match state version"
                        .to_string(),
                ));
            }
            if last_audit.checksum != bundle.state.state_checksum {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest audit checksum does not match state checksum"
                        .to_string(),
                ));
            }
        }

        if bundle
            .snapshots
            .iter()
            .any(|snapshot| snapshot.version != snapshot.state.state_version)
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: snapshot version does not match embedded state version"
                    .to_string(),
            ));
        }

        if let Some(last_snapshot) = bundle.snapshots.last() {
            if last_snapshot.version != bundle.state.state_version {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest snapshot version does not match state version"
                        .to_string(),
                ));
            }
            if last_snapshot.state.state_checksum != bundle.state.state_checksum {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest snapshot checksum does not match state checksum"
                        .to_string(),
                ));
            }
        }

        if let (Some(last_audit), Some(last_snapshot)) =
            (bundle.audit_entries.last(), bundle.snapshots.last())
            && last_audit.version != last_snapshot.version
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: latest audit and snapshot versions diverge"
                    .to_string(),
            ));
        }

        let snapshot_checksums_by_version: std::collections::HashMap<u64, &str> = bundle
            .snapshots
            .iter()
            .map(|snapshot| (snapshot.version, snapshot.state.state_checksum.as_str()))
            .collect();

        let audit_versions: std::collections::HashSet<u64> = bundle
            .audit_entries
            .iter()
            .map(|entry| entry.version)
            .collect();
        if bundle
            .snapshots
            .iter()
            .any(|snapshot| !audit_versions.contains(&snapshot.version))
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: snapshot version missing matching audit entry"
                    .to_string(),
            ));
        }

        if bundle.audit_entries.iter().any(|audit| {
            snapshot_checksums_by_version
                .get(&audit.version)
                .is_some_and(|snapshot_checksum| *snapshot_checksum != audit.checksum.as_str())
        }) {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: audit checksum diverges from snapshot checksum for same version"
                    .to_string(),
            ));
        }

        Ok(())
    }

    fn arbitration_score(
        &self,
        output: &ExpertOutput,
        routing_scores: &std::collections::HashMap<ExpertId, f64>,
    ) -> f64 {
        match self.arbitration_mode {
            ArbitrationMode::Aggregation => output.confidence,
            ArbitrationMode::RouterScoreWeighted => {
                output.confidence
                    + routing_scores
                        .get(&output.expert_id)
                        .copied()
                        .unwrap_or(0.0)
            }
        }
    }

    fn pick_policy_compliant_output(
        &self,
        outputs: &[ExpertOutput],
        routing_scores: &std::collections::HashMap<ExpertId, f64>,
        exclude_expert_id: Option<&ExpertId>,
    ) -> Option<ExpertOutput> {
        let mut ranked: Vec<&ExpertOutput> = outputs.iter().collect();
        ranked.sort_by(|a, b| {
            self.arbitration_score(b, routing_scores)
                .partial_cmp(&self.arbitration_score(a, routing_scores))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        ranked
            .into_iter()
            .filter(|output| exclude_expert_id != Some(&output.expert_id))
            .find(|output| self.policy_guard.validate_strict(output).is_ok())
            .cloned()
    }

    fn record_governance_audit(&mut self, reason: &str) {
        self.governance_state_version = self.governance_state_version.saturating_add(1);
        let state = self.export_governance_state();
        let checksum = state.state_checksum.clone();
        self.governance_audit_entries.push(GovernanceAuditEntry {
            version: self.governance_state_version,
            checksum,
            reason: reason.to_string(),
        });
        if self.governance_audit_entries.len() > self.max_governance_audit_entries {
            let to_trim = self.governance_audit_entries.len() - self.max_governance_audit_entries;
            self.governance_audit_entries.drain(0..to_trim);
        }

        self.governance_state_snapshots
            .push(GovernanceStateSnapshot {
                version: self.governance_state_version,
                reason: reason.to_string(),
                state,
            });
        if self.governance_state_snapshots.len() > self.max_governance_state_snapshots {
            let to_trim =
                self.governance_state_snapshots.len() - self.max_governance_state_snapshots;
            self.governance_state_snapshots.drain(0..to_trim);
        }
        self.retain_snapshots_with_matching_audit_versions();
    }

    fn evaluate_governance_import(&self, state: &GovernanceState) -> GovernanceImportDecision {
        let diff = self.diff_governance_state(state);
        let mut reasons = Vec::new();

        if !self.governance_import_policy.allow_schema_change && diff.schema_version_changed {
            reasons.push("schema version drift is not allowed".to_string());
        }
        if !self.governance_import_policy.allow_version_regression && diff.version_delta < 0 {
            reasons.push("version regression is not allowed".to_string());
        }
        if let Some(max) = self.governance_import_policy.max_version_regression
            && diff.version_delta < 0
            && (-diff.version_delta as u64) > max
        {
            reasons.push("version regression exceeds configured maximum".to_string());
        }
        if self.governance_import_policy.require_policy_match && diff.policy_changed {
            reasons.push("governance policy mismatch".to_string());
        }

        GovernanceImportDecision {
            allowed: reasons.is_empty(),
            reasons,
            diff,
        }
    }

    fn retain_snapshots_with_matching_audit_versions(&mut self) {
        if self.governance_state_snapshots.is_empty() || self.governance_audit_entries.is_empty() {
            self.governance_state_snapshots.clear();
            return;
        }

        let audit_versions: std::collections::HashSet<u64> = self
            .governance_audit_entries
            .iter()
            .map(|entry| entry.version)
            .collect();
        self.governance_state_snapshots
            .retain(|snapshot| audit_versions.contains(&snapshot.version));
    }
}
