use crate::aggregator::{AggregationStrategy, OutputAggregator};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, MoeError, Task, TaskType, TracePhase,
};
use crate::orchestrator::MoePipelineBuilder;
use crate::router::{HeuristicRouter, Router};
use std::collections::HashMap;

struct RoutingExpert {
    metadata: ExpertMetadata,
    confidence: f64,
    fail: bool,
}

impl RoutingExpert {
    fn new(id: &str, capabilities: Vec<ExpertCapability>, confidence: f64, fail: bool) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::new(id),
                name: id.to_string(),
                version: "1.0.0".to_string(),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
            confidence,
            fail,
        }
    }
}

impl Expert for RoutingExpert {
    fn id(&self) -> &ExpertId {
        &self.metadata.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.metadata
    }

    fn can_handle(&self, task: &Task) -> bool {
        matches!(task.task_type(), TaskType::CodeGeneration)
    }

    fn execute(
        &self,
        task: &Task,
        _context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        if self.fail {
            return Err(ExpertError::ExecutionFailed("forced failure".to_string()));
        }
        Ok(ExpertOutput {
            expert_id: self.metadata.id.clone(),
            content: format!("{}:{}", self.metadata.name, task.input()),
            confidence: self.confidence,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

#[test]
fn v1_heuristic_router_selects_matching_experts() {
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(3)))
        .build();
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "code-a",
            vec![ExpertCapability::CodeGeneration],
            0.9,
            false,
        )))
        .expect("registering first expert should succeed");
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "code-b",
            vec![ExpertCapability::CodeGeneration],
            0.8,
            false,
        )))
        .expect("registering second expert should succeed");

    let task = Task::new("v1-router", TaskType::CodeGeneration, "build");
    let decision = HeuristicRouter::new(3)
        .route(&task, pipeline.registry())
        .expect("routing should succeed");
    assert!(!decision.selected_experts.is_empty());
}

#[test]
fn v1_duplicate_expert_registration_returns_error() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "dup",
            vec![ExpertCapability::CodeGeneration],
            0.9,
            false,
        )))
        .expect("initial registration should succeed");
    let duplicate = pipeline.register_expert(Box::new(RoutingExpert::new(
        "dup",
        vec![ExpertCapability::CodeGeneration],
        0.9,
        false,
    )));
    assert!(duplicate.is_err());
}

#[test]
fn v1_single_expert_execution_runs_successfully() {
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(1)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .build();
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "single",
            vec![ExpertCapability::CodeGeneration],
            0.95,
            false,
        )))
        .expect("expert registration should succeed");

    let task = Task::new("v1-single", TaskType::CodeGeneration, "single run");
    let result = pipeline.execute(task).expect("pipeline should succeed");
    assert_eq!(result.outputs.len(), 1);
    let selected = result
        .selected_output
        .expect("selected output should be present");
    assert_eq!(selected.expert_id.as_str(), "single");
}

#[test]
fn v1_multi_expert_execution_runs_successfully() {
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(3)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .build();
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "multi-a",
            vec![ExpertCapability::CodeGeneration],
            0.7,
            false,
        )))
        .expect("first expert registration should succeed");
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "multi-b",
            vec![ExpertCapability::CodeGeneration],
            0.9,
            false,
        )))
        .expect("second expert registration should succeed");

    let task = Task::new("v1-multi", TaskType::CodeGeneration, "multi run");
    let result = pipeline.execute(task).expect("pipeline should succeed");
    assert!(result.outputs.len() >= 2);
}

#[test]
fn v1_routing_traces_are_recorded() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "trace-a",
            vec![ExpertCapability::CodeGeneration],
            0.9,
            false,
        )))
        .expect("expert registration should succeed");

    let task = Task::new("v1-trace", TaskType::CodeGeneration, "trace run");
    let _ = pipeline.execute(task).expect("pipeline should succeed");
    let routing_traces = pipeline.trace_logger().get_by_phase(&TracePhase::Routing);
    assert!(!routing_traces.is_empty());
}

#[test]
fn v1_expert_failure_is_propagated() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(RoutingExpert::new(
            "fail-a",
            vec![ExpertCapability::CodeGeneration],
            0.9,
            true,
        )))
        .expect("expert registration should succeed");

    let task = Task::new("v1-fail", TaskType::CodeGeneration, "fail run");
    let error = pipeline
        .execute(task)
        .expect_err("pipeline should return an expert failure");
    assert!(matches!(error, MoeError::ExpertError(_)));
}

#[test]
fn v1_basic_aggregation_works() {
    let aggregator = OutputAggregator::new(AggregationStrategy::HighestConfidence);
    let outputs = vec![
        ExpertOutput {
            expert_id: ExpertId::new("a"),
            content: "a".to_string(),
            confidence: 0.6,
            metadata: HashMap::new(),
            trace: Vec::new(),
        },
        ExpertOutput {
            expert_id: ExpertId::new("b"),
            content: "b".to_string(),
            confidence: 0.9,
            metadata: HashMap::new(),
            trace: Vec::new(),
        },
    ];

    let aggregated = aggregator
        .aggregate(outputs)
        .expect("aggregation should succeed");
    let selected = aggregated
        .selected_output
        .expect("selected output should be present");
    assert_eq!(selected.expert_id.as_str(), "b");
}
