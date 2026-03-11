use crate::aggregator::AggregationStrategy;
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
};
use crate::orchestrator::MoePipelineBuilder;
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
