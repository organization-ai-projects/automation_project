use crate::aggregator::{AggregationStrategy, OutputAggregator};
use crate::dataset_engine::{DatasetStore, TraceConverter};
use crate::evaluation_engine::EvaluationEngine;
use crate::expert_registry::ExpertRegistry;
use crate::feedback_engine::FeedbackStore;
use crate::orchestrator::ArbitrationMode;
use crate::policy_guard::PolicyGuard;
use crate::router::{HeuristicRouter, Router};
use crate::trace_logger::TraceLogger;

use super::moe_pipeline_core::MoePipeline;

pub struct MoePipelineBuilder {
    router: Option<Box<dyn Router>>,
    aggregation_strategy: AggregationStrategy,
    arbitration_mode: ArbitrationMode,
    fallback_on_expert_error: bool,
    enable_task_metadata_chain: bool,
    max_traces: usize,
}

impl MoePipelineBuilder {
    pub fn new() -> Self {
        Self {
            router: None,
            aggregation_strategy: AggregationStrategy::HighestConfidence,
            arbitration_mode: ArbitrationMode::Aggregation,
            fallback_on_expert_error: false,
            enable_task_metadata_chain: false,
            max_traces: 10_000,
        }
    }

    pub fn with_router(mut self, router: Box<dyn Router>) -> Self {
        self.router = Some(router);
        self
    }

    pub fn with_aggregation_strategy(mut self, strategy: AggregationStrategy) -> Self {
        self.aggregation_strategy = strategy;
        self
    }

    pub fn with_arbitration_mode(mut self, mode: ArbitrationMode) -> Self {
        self.arbitration_mode = mode;
        self
    }

    pub fn with_fallback_on_expert_error(mut self, enabled: bool) -> Self {
        self.fallback_on_expert_error = enabled;
        self
    }

    pub fn with_task_metadata_chain(mut self, enabled: bool) -> Self {
        self.enable_task_metadata_chain = enabled;
        self
    }

    pub fn with_max_traces(mut self, max: usize) -> Self {
        self.max_traces = max;
        self
    }

    pub fn build(self) -> MoePipeline {
        let router = self
            .router
            .unwrap_or_else(|| Box::new(HeuristicRouter::default()));

        MoePipeline {
            registry: ExpertRegistry::new(),
            router,
            aggregator: OutputAggregator::new(self.aggregation_strategy),
            arbitration_mode: self.arbitration_mode,
            fallback_on_expert_error: self.fallback_on_expert_error,
            enable_task_metadata_chain: self.enable_task_metadata_chain,
            policy_guard: PolicyGuard::new(),
            trace_logger: TraceLogger::new(self.max_traces),
            evaluation: EvaluationEngine::new(),
            evaluation_baseline: None,
            feedback_store: FeedbackStore::new(),
            dataset_store: DatasetStore::new(),
            trace_converter: TraceConverter::new(),
        }
    }
}

impl Default for MoePipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
