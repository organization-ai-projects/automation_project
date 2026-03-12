use crate::aggregator::AggregationStrategy;
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, MoeError, Task, TaskType,
};
use crate::orchestrator::MoePipelineBuilder;
use crate::retrieval_engine::{RetrievalQuery, RetrievalResult, Retriever};
use std::collections::HashMap;

struct TestExpert {
    meta: ExpertMetadata,
}

impl TestExpert {
    fn new(id: &str, capabilities: Vec<ExpertCapability>) -> Self {
        Self {
            meta: ExpertMetadata {
                id: ExpertId::new(id),
                name: id.to_string(),
                version: "1.0.0".to_string(),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

struct StubRetriever;

impl Retriever for StubRetriever {
    fn retrieve(&self, _query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError> {
        Ok(vec![RetrievalResult::new(
            "ctx-1",
            "retrieved rust context",
            0.9,
            "doc://ctx",
        )])
    }
}

impl Expert for TestExpert {
    fn id(&self) -> &ExpertId {
        &self.meta.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.meta
    }

    fn can_handle(&self, task: &Task) -> bool {
        matches!(task.task_type(), TaskType::CodeGeneration)
    }

    fn execute(
        &self,
        task: &Task,
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        let trace_hint = context.retrieved_context.len() + context.memory_entries.len();
        Ok(ExpertOutput {
            expert_id: self.meta.id.clone(),
            content: format!("{}:{trace_hint}", task.input()),
            confidence: 0.95,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

#[test]
fn builder_builds_pipeline() {
    let pipeline = MoePipelineBuilder::new()
        .with_aggregation_strategy(AggregationStrategy::First)
        .with_max_traces(500)
        .build();
    assert_eq!(pipeline.registry().count(), 0);
}

#[test]
fn register_expert() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let expert = TestExpert::new("e1", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");
    assert_eq!(pipeline.registry().count(), 1);
}

#[test]
fn full_execute_pipeline() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let expert = TestExpert::new("codegen", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");

    let task = Task::new("t1", TaskType::CodeGeneration, "write code");
    let result = pipeline
        .execute(task)
        .expect("pipeline execution should succeed");
    assert!(result.selected_output.is_some());
    let selected = result
        .selected_output
        .expect("selected output should be present");
    assert_eq!(selected.expert_id.as_str(), "codegen");
    assert!(pipeline.trace_logger().count() > 0);
}

#[test]
fn execute_enriches_context_with_retrieval_memory_and_buffer() {
    let mut pipeline = MoePipelineBuilder::new()
        .with_retriever(Box::new(StubRetriever))
        .with_context_max_length(256)
        .build();
    let expert = TestExpert::new("codegen", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");

    pipeline
        .remember_short_term(MemoryEntry {
            id: "mem-1".to_string(),
            content: "recent memory".to_string(),
            tags: vec!["runtime".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("short-term memory write should succeed");

    let task = Task::new("t-enriched", TaskType::CodeGeneration, "write rust code");
    let result = pipeline
        .execute(task)
        .expect("pipeline execution should succeed");
    let selected = result
        .selected_output
        .expect("selected output should be present");

    // TestExpert emits "<input>:<retrieved+memory>"; task-aware assembly adds a header segment.
    assert!(selected.content.ends_with(":3"));

    let retrieval_traces = pipeline
        .trace_logger()
        .get_by_phase(&crate::moe_core::TracePhase::Retrieval);
    let memory_traces = pipeline
        .trace_logger()
        .get_by_phase(&crate::moe_core::TracePhase::MemoryQuery);
    assert!(!retrieval_traces.is_empty());
    assert!(!memory_traces.is_empty());
}
