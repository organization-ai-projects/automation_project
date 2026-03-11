//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/moe_pipeline.rs
use crate::aggregator::OutputAggregator;
use crate::dataset_engine::{DatasetStore, Outcome, TraceConverter};
use crate::evaluation_engine::EvaluationEngine;
use crate::expert_registry::ExpertRegistry;
use crate::feedback_engine::{FeedbackEntry, FeedbackStore};
use crate::moe_core::{
    AggregatedOutput, ExecutionContext, Expert, ExpertError, ExpertId, ExpertOutput, MoeError,
    Task, TracePhase,
};
use crate::orchestrator::ContinuousImprovementReport;
use crate::orchestrator::{
    ArbitrationMode, ContinuousGovernancePolicy, GovernanceAuditEntry, GovernanceAuditTrail,
    GovernanceImportDecision, GovernanceImportPolicy, GovernanceState, GovernanceStateDiff,
};
use crate::policy_guard::{Policy, PolicyGuard};
use crate::router::{Router, RoutingStrategy};
use crate::trace_logger::TraceLogger;

pub struct MoePipeline {
    pub(super) registry: ExpertRegistry,
    pub(super) router: Box<dyn Router>,
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

        // 3. Build execution context (placeholder for retrieval/memory integration)
        let context = ExecutionContext::new(task_id.clone());

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

    pub fn import_governance_state_json(&mut self, payload: &str) -> Result<(), MoeError> {
        self.try_import_governance_state_json(payload)
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

    fn parse_expert_chain(raw: &str) -> Vec<ExpertId> {
        raw.split([',', '>'])
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(ExpertId::new)
            .collect()
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
        let checksum = self.export_governance_state().state_checksum;
        self.governance_audit_entries.push(GovernanceAuditEntry {
            version: self.governance_state_version,
            checksum,
            reason: reason.to_string(),
        });
        if self.governance_audit_entries.len() > self.max_governance_audit_entries {
            let to_trim = self.governance_audit_entries.len() - self.max_governance_audit_entries;
            self.governance_audit_entries.drain(0..to_trim);
        }
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
}
