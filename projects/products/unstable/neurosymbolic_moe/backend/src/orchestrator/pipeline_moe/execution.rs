//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/execution.rs
use std::{cmp, collections};

use protocol::ProtocolId;

use crate::dataset_engine;
use crate::memory_engine::{MemoryQuery, MemoryStore};
use crate::moe_core::{
    AggregatedOutput, ExecutionContext, ExpertError, ExpertId, ExpertOutput, MoeError, Task,
    TaskId, TracePhase,
};
use crate::orchestrator::{
    ArbitrationMode, ContinuousImprovementReport, MoePipeline, TrainerTriggerEvent, Version,
};
use crate::retrieval_engine::RetrievalQuery;
use crate::router::RoutingStrategy;

impl MoePipeline {
    fn parse_expert_chain(&self, raw: &str) -> Vec<ExpertId> {
        raw.split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .filter_map(|entry| self.registry.find_id_by_name_or_id(entry))
            .collect()
    }

    pub fn execute(&mut self, task: Task) -> Result<AggregatedOutput, MoeError> {
        let task_id = task.id().clone();
        let chain_ids = if self.enable_task_metadata_chain {
            task.metadata
                .get("expert_chain")
                .map(|raw| self.parse_expert_chain(raw))
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let chain_enabled = !chain_ids.is_empty();

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
                    collections::HashMap::new(),
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

        let context = self.build_execution_context(&task, &selected_experts)?;

        let mut outputs: Vec<ExpertOutput> = Vec::new();
        let mut chain_input: Option<String> = None;

        for (index, expert_id) in selected_experts.iter().enumerate() {
            let expert = self.registry.get(expert_id).ok_or_else(|| {
                let msg = format!("expert '{}' not found in registry", expert_id);
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
                                expert_id
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
                            expert_id, output.confidence
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
                        format!("expert '{}' failed: {e}", expert_id),
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
                        .unwrap_or(cmp::Ordering::Equal)
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
                            selected.expert_id, replacement.expert_id
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

        let entry = self.training_runtime_state.trace_converter.convert(
            &task_traces,
            task.input(),
            output_text,
            dataset_engine::Outcome::Success,
        );

        self.training_runtime_state.dataset_store.add_entry(entry);

        self.trace_logger.log_phase(
            task_id,
            TracePhase::DatasetEnrichment,
            "traces converted to dataset entry".to_string(),
            None,
        );

        if let Some(policy) = &self.governance_runtime_state.continuous_governance_policy {
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
                    self.governance_runtime_state
                        .last_continuous_improvement_report = Some(report);
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

            self.governance_runtime_state
                .last_continuous_improvement_report = Some(report);
            self.record_governance_audit("continuous governance evaluated");
        }
        self.maybe_run_auto_improvement();
        self.validate_runtime_invariants()?;

        Ok(aggregated)
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
        let dataset_quality = self
            .training_runtime_state
            .dataset_store
            .quality_report(low_score_threshold);

        let (expert_regressions, routing_regression) =
            if let Some(baseline) = &self.governance_runtime_state.evaluation_baseline {
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

    fn maybe_run_auto_improvement(&mut self) {
        let Some(policy) = self.training_runtime_state.auto_improvement_policy.clone() else {
            return;
        };
        if self.training_runtime_state.dataset_store.count() < policy.min_dataset_entries {
            self.training_runtime_state
                .auto_improvement_status
                .skip_counters
                .min_dataset_entries_total += 1;
            self.training_runtime_state
                .auto_improvement_status
                .last_skip_reason = Some("dataset entries below min_dataset_entries".to_string());
            return;
        }

        let quality = self
            .training_runtime_state
            .dataset_store
            .quality_report(policy.training_build_options.min_score.unwrap_or(0.0));
        if quality.success_ratio < policy.min_success_ratio {
            self.training_runtime_state
                .auto_improvement_status
                .skip_counters
                .min_success_ratio_total += 1;
            self.training_runtime_state
                .auto_improvement_status
                .last_skip_reason =
                Some("dataset success_ratio below min_success_ratio".to_string());
            return;
        }
        if let Some(min_average_score) = policy.min_average_score
            && quality.average_score.unwrap_or(0.0) < min_average_score
        {
            self.training_runtime_state
                .auto_improvement_status
                .skip_counters
                .min_average_score_total += 1;
            self.training_runtime_state
                .auto_improvement_status
                .last_skip_reason =
                Some("dataset average_score below min_average_score".to_string());
            return;
        }
        if self
            .governance_runtime_state
            .last_continuous_improvement_report
            .as_ref()
            .is_some_and(|report| report.requires_human_review)
        {
            self.training_runtime_state
                .auto_improvement_status
                .skip_counters
                .human_review_required_total += 1;
            self.training_runtime_state
                .auto_improvement_status
                .last_skip_reason = Some("continuous governance requires human review".to_string());
            return;
        }

        match self
            .training_runtime_state
            .dataset_store
            .build_training_bundle_with_provenance(
                &policy.training_build_options,
                self.dataset_provenance(),
            ) {
            Ok(bundle) => {
                if self
                    .training_runtime_state
                    .auto_improvement_status
                    .last_bundle_checksum
                    .as_deref()
                    == Some(bundle.bundle_checksum.as_str())
                {
                    self.training_runtime_state
                        .auto_improvement_status
                        .skip_counters
                        .duplicate_bundle_total += 1;
                    self.training_runtime_state
                        .auto_improvement_status
                        .last_skip_reason = Some("training bundle checksum unchanged".to_string());
                    return;
                }
                let model_version: Version = self
                    .training_runtime_state
                    .model_registry
                    .register_candidate(
                        bundle.bundle_checksum.clone(),
                        bundle.included_entries,
                        bundle.train_samples.len(),
                        bundle.validation_samples.len(),
                        bundle.generated_at,
                    );
                if self
                    .training_runtime_state
                    .model_registry
                    .active_model_version
                    .is_none()
                {
                    self.training_runtime_state
                        .model_registry
                        .promote(model_version.clone());
                }
                self.training_runtime_state
                    .auto_improvement_status
                    .global_counters
                    .runs_total += 1;
                self.training_runtime_state
                    .auto_improvement_status
                    .last_bundle_checksum = Some(bundle.bundle_checksum.clone());
                self.training_runtime_state
                    .auto_improvement_status
                    .last_included_entries = bundle.included_entries;
                self.training_runtime_state
                    .auto_improvement_status
                    .last_train_samples = bundle.train_samples.len();
                self.training_runtime_state
                    .auto_improvement_status
                    .last_validation_samples = bundle.validation_samples.len();
                self.training_runtime_state
                    .auto_improvement_status
                    .last_skip_reason = None;
                self.push_trainer_trigger_event(TrainerTriggerEvent {
                    event_id: ProtocolId::default(),
                    model_version: model_version.clone(),
                    training_bundle_checksum: bundle.bundle_checksum.clone(),
                    included_entries: bundle.included_entries,
                    train_samples: bundle.train_samples.len(),
                    validation_samples: bundle.validation_samples.len(),
                    generated_at: bundle.generated_at,
                    delivery_attempts: 0,
                    last_attempted_at: None,
                });
                self.record_governance_audit("auto improvement dataset refresh");
                self.trace_logger.log_phase(
                    TaskId::new(),
                    TracePhase::DatasetEnrichment,
                    format!(
                        "auto improvement run {} prepared model v{} bundle (included={}, train={}, validation={})",
                        self.training_runtime_state
                            .auto_improvement_status
                            .global_counters
                            .runs_total,
                        model_version,
                        self.training_runtime_state.auto_improvement_status.last_included_entries,
                        self.training_runtime_state.auto_improvement_status.last_train_samples,
                        self.training_runtime_state.auto_improvement_status.last_validation_samples
                    ),
                    None,
                );
            }
            Err(err) => {
                self.training_runtime_state
                    .auto_improvement_status
                    .global_counters
                    .build_failures_total += 1;
                self.training_runtime_state
                    .auto_improvement_status
                    .last_skip_reason = Some(format!("training bundle build error: {err}"));
                self.trace_logger.log_phase(
                    TaskId::new(),
                    TracePhase::DatasetEnrichment,
                    format!("auto improvement skipped (bundle build error: {err})"),
                    None,
                );
            }
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

        let working_key = format!("task/{}", task.id());
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
        if let Some(session_id) = task
            .metadata
            .get("session_id")
            .and_then(|raw| raw.parse::<ProtocolId>().ok())
        {
            buffer_data.extend(self.buffer_manager.sessions().values(&session_id));
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

    fn arbitration_score(
        &self,
        output: &ExpertOutput,
        routing_scores: &collections::HashMap<ExpertId, f64>,
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
        routing_scores: &collections::HashMap<ExpertId, f64>,
        exclude_expert_id: Option<&ExpertId>,
    ) -> Option<ExpertOutput> {
        let mut ranked: Vec<&ExpertOutput> = outputs.iter().collect();
        ranked.sort_by(|a, b| {
            self.arbitration_score(b, routing_scores)
                .partial_cmp(&self.arbitration_score(a, routing_scores))
                .unwrap_or(cmp::Ordering::Equal)
        });

        ranked
            .into_iter()
            .filter(|output| exclude_expert_id != Some(&output.expert_id))
            .find(|output| self.policy_guard.validate_strict(output).is_ok())
            .cloned()
    }
}
