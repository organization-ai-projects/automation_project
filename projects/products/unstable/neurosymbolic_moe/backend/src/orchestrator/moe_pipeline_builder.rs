use crate::aggregator::{AggregationStrategy, OutputAggregator};
use crate::buffer_manager::BufferManager;
use crate::dataset_engine::{DatasetStore, TraceConverter};
use crate::evaluation_engine::EvaluationEngine;
use crate::expert_registry::ExpertRegistry;
use crate::feedback_engine::FeedbackStore;
use crate::memory_engine::{LongTermMemory, ShortTermMemory};
use crate::orchestrator::ImportTelemetry;
use crate::orchestrator::import_journal::ImportJournal;
use crate::orchestrator::{ArbitrationMode, ContinuousGovernancePolicy, GovernanceImportPolicy};
use crate::policy_guard::PolicyGuard;
use crate::retrieval_engine::{ContextAssembler, Retriever, SimpleRetriever};
use crate::router::{HeuristicRouter, Router};
use crate::trace_logger::TraceLogger;

use super::moe_pipeline::MoePipeline;

pub struct MoePipelineBuilder {
    router: Option<Box<dyn Router>>,
    aggregation_strategy: AggregationStrategy,
    arbitration_mode: ArbitrationMode,
    fallback_on_expert_error: bool,
    enable_task_metadata_chain: bool,
    retriever: Option<Box<dyn Retriever>>,
    context_max_length: usize,
    short_term_memory_capacity: usize,
    working_buffer_capacity: usize,
    continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    governance_import_policy: GovernanceImportPolicy,
    max_governance_audit_entries: usize,
    max_governance_state_snapshots: usize,
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
            retriever: None,
            context_max_length: 1_024,
            short_term_memory_capacity: 256,
            working_buffer_capacity: 128,
            continuous_governance_policy: None,
            governance_import_policy: GovernanceImportPolicy::default(),
            max_governance_audit_entries: 128,
            max_governance_state_snapshots: 64,
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

    pub fn with_retriever(mut self, retriever: Box<dyn Retriever>) -> Self {
        self.retriever = Some(retriever);
        self
    }

    pub fn with_context_max_length(mut self, max_len: usize) -> Self {
        self.context_max_length = max_len;
        self
    }

    pub fn with_continuous_governance_policy(mut self, policy: ContinuousGovernancePolicy) -> Self {
        self.continuous_governance_policy = Some(policy);
        self
    }

    pub fn with_governance_import_policy(mut self, policy: GovernanceImportPolicy) -> Self {
        self.governance_import_policy = policy;
        self
    }

    pub fn with_max_governance_audit_entries(mut self, max: usize) -> Self {
        self.max_governance_audit_entries = max;
        self
    }

    pub fn with_max_governance_state_snapshots(mut self, max: usize) -> Self {
        self.max_governance_state_snapshots = max;
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
        let retriever = self
            .retriever
            .unwrap_or_else(|| Box::new(SimpleRetriever::new()));

        MoePipeline {
            registry: ExpertRegistry::new(),
            router,
            retriever,
            context_assembler: ContextAssembler::new(self.context_max_length),
            short_term_memory: ShortTermMemory::new(self.short_term_memory_capacity),
            long_term_memory: LongTermMemory::new(),
            buffer_manager: BufferManager::new(self.working_buffer_capacity),
            aggregator: OutputAggregator::new(self.aggregation_strategy),
            arbitration_mode: self.arbitration_mode,
            fallback_on_expert_error: self.fallback_on_expert_error,
            enable_task_metadata_chain: self.enable_task_metadata_chain,
            continuous_governance_policy: self.continuous_governance_policy,
            governance_import_policy: self.governance_import_policy,
            policy_guard: PolicyGuard::new(),
            trace_logger: TraceLogger::new(self.max_traces),
            evaluation: EvaluationEngine::new(),
            evaluation_baseline: None,
            last_continuous_improvement_report: None,
            governance_state_version: 0,
            governance_audit_entries: Vec::new(),
            max_governance_audit_entries: self.max_governance_audit_entries,
            governance_state_snapshots: Vec::new(),
            max_governance_state_snapshots: self.max_governance_state_snapshots,
            import_telemetry: ImportTelemetry::default(),
            import_journal: ImportJournal::with_capacity(256),
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
