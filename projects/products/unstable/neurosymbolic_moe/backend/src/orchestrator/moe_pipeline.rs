use crate::aggregator::OutputAggregator;
use crate::dataset_engine::{DatasetStore, Outcome, TraceConverter};
use crate::evaluation_engine::EvaluationEngine;
use crate::expert_registry::ExpertRegistry;
use crate::feedback_engine::{FeedbackEntry, FeedbackStore};
use crate::moe_core::{
    AggregatedOutput, ExecutionContext, Expert, ExpertOutput, MoeError, Task, TracePhase,
};
use crate::policy_guard::{Policy, PolicyGuard};
use crate::router::{Router, RoutingStrategy};
use crate::trace_logger::TraceLogger;

pub struct MoePipeline {
    pub(super) registry: ExpertRegistry,
    pub(super) router: Box<dyn Router>,
    pub(super) aggregator: OutputAggregator,
    pub(super) policy_guard: PolicyGuard,
    pub(super) trace_logger: TraceLogger,
    pub(super) evaluation: EvaluationEngine,
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

        // 1. Routing phase
        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::Routing,
            "routing phase begins".to_string(),
            None,
        );

        let decision = self.router.route(&task, &self.registry).map_err(|e| {
            self.trace_logger.log_phase(
                task_id.clone(),
                TracePhase::Routing,
                format!("routing failed: {e}"),
                None,
            );
            e
        })?;

        let used_fallback = matches!(decision.strategy, RoutingStrategy::Fallback);
        self.evaluation
            .record_routing(decision.selected_experts.len(), used_fallback);

        // 2. Expert selection
        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::ExpertSelection,
            format!(
                "selected {} expert(s): {}",
                decision.selected_experts.len(),
                decision.explanation
            ),
            None,
        );

        // 3. Build execution context (placeholder for retrieval/memory integration)
        let context = ExecutionContext::new(task_id.clone());

        // 4. Execute selected experts
        let mut outputs: Vec<ExpertOutput> = Vec::new();

        for expert_id in &decision.selected_experts {
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

            match expert.execute(&task, &context) {
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
                    return Err(MoeError::ExpertError(e));
                }
            }
        }

        // 5. Aggregation
        let aggregated = self.aggregator.aggregate(outputs).map_err(|e| {
            self.trace_logger.log_phase(
                task_id.clone(),
                TracePhase::Aggregation,
                format!("aggregation failed: {e}"),
                None,
            );
            e
        })?;

        self.trace_logger.log_phase(
            task_id.clone(),
            TracePhase::Aggregation,
            format!("aggregated with strategy '{}'", aggregated.strategy),
            None,
        );

        // 6. Policy validation
        if let Some(ref selected) = aggregated.selected_output {
            self.policy_guard.validate_strict(selected).map_err(|e| {
                self.trace_logger.log_phase(
                    task_id.clone(),
                    TracePhase::Validation,
                    format!("policy validation failed: {e}"),
                    None,
                );
                e
            })?;
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
}
