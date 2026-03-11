use crate::aggregator::{AggregationStrategy, OutputAggregator};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType, TracePhase, TraceRecord,
};
use crate::orchestrator::MoePipelineBuilder;
use crate::router::{HeuristicRouter, Router};
use std::collections::HashMap;

struct V0Expert {
    metadata: ExpertMetadata,
}

impl V0Expert {
    fn new(id: &str) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::new(id),
                name: "v0-expert".to_string(),
                version: "0.1.0".to_string(),
                capabilities: vec![ExpertCapability::CodeGeneration],
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

impl Expert for V0Expert {
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
        Ok(ExpertOutput {
            expert_id: self.metadata.id.clone(),
            content: format!("v0:{}", task.input()),
            confidence: 1.0,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

#[test]
fn v0_core_contracts_are_wired() {
    let expert_port: &dyn Expert = &V0Expert::new("expert-v0");
    let router = HeuristicRouter::default();
    let router_port: &dyn Router = &router;

    let _ = expert_port.id();
    let _ = router_port;
}

#[test]
fn v0_models_and_aggregation_flow_work() {
    let task = Task::new("task-v0", TaskType::CodeGeneration, "hello");
    let output = ExpertOutput {
        expert_id: ExpertId::new("expert-v0"),
        content: "ok".to_string(),
        confidence: 0.8,
        metadata: HashMap::new(),
        trace: Vec::new(),
    };
    let aggregator = OutputAggregator::new(AggregationStrategy::First);
    let aggregated = aggregator
        .aggregate(vec![output])
        .expect("v0 aggregation should succeed");
    assert_eq!(task.id().as_str(), "task-v0");
    assert_eq!(aggregated.outputs.len(), 1);
    assert_eq!(aggregated.strategy, "first");
}

#[test]
fn v0_trace_model_is_usable() {
    let trace = TraceRecord {
        trace_id: "trace-v0".to_string(),
        task_id: crate::moe_core::TaskId::new("task-v0"),
        timestamp: 1,
        expert_id: Some(ExpertId::new("expert-v0")),
        phase: TracePhase::Routing,
        detail: "routed".to_string(),
        metadata: HashMap::new(),
    };
    assert_eq!(trace.trace_id, "trace-v0");
    assert!(matches!(trace.phase, TracePhase::Routing));
}

#[test]
fn v0_minimal_orchestration_flow_executes() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(V0Expert::new("expert-v0")))
        .expect("registering v0 expert should succeed");

    let task = Task::new("task-v0", TaskType::CodeGeneration, "run");
    let result = pipeline
        .execute(task)
        .expect("v0 orchestration should succeed");
    assert!(result.selected_output.is_some());
    assert!(pipeline.trace_logger().count() > 0);
}
