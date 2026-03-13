use crate::echo_expert::EchoExpert;
use crate::moe_core::{ExpertCapability, Task, TaskType};
use crate::orchestrator::MoePipelineBuilder;

#[test]
fn execution_module_runs_pipeline_with_registered_expert() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(EchoExpert::new(
            "execution_mod_echo",
            "ExecutionModEcho",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("expert registration should succeed");

    let result = pipeline
        .execute(Task::new(
            "execution-mod-task",
            TaskType::CodeGeneration,
            "execution module payload",
        ))
        .expect("execution should succeed");
    assert!(result.selected_output.is_some());
}

#[test]
fn execution_module_builds_continuous_improvement_report() {
    let pipeline = MoePipelineBuilder::new().build();
    let report = pipeline.continuous_improvement_report(0.5, 0.5, 0.4, 0.2);
    assert!(report.governance.underperforming_experts.is_empty());
    assert!(!report.requires_human_review);
}
