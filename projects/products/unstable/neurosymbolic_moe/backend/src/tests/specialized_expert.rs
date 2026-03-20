use crate::moe_core::{ExecutionContext, Expert, Task, TaskType};
use crate::specialized_expert::SpecializedExpert;

#[test]
fn planning_expert_handles_planning_but_not_validation_tasks() {
    let expert = SpecializedExpert::planning("PlanningExpert");
    let planning_task = Task::new(TaskType::Planning, "plan the rollout");
    let validation_task = Task::new(TaskType::Validation, "validate the rollout");

    assert!(expert.can_handle(&planning_task));
    assert!(!expert.can_handle(&validation_task));
}

#[test]
fn validation_expert_emits_validation_specific_output() {
    let expert = SpecializedExpert::validation("ValidationExpert");
    let task = Task::new(TaskType::Validation, "check migration safety");
    let context = ExecutionContext::new(task.id().clone())
        .with_retrieved_context(vec!["governance diff".to_string()])
        .with_memory_entries(vec!["previous rollback".to_string()]);

    let output = expert.execute(&task, &context).unwrap();

    assert!(output.content.contains("Validation report"));
    assert!(output.content.contains("policy or consistency risks"));
    assert!(output.confidence >= 0.95);
    assert_eq!(
        output.metadata.get("expert_role").map(String::as_str),
        Some("ValidationExpert")
    );
}

#[test]
fn code_generation_and_transformation_experts_produce_different_outputs() {
    let generation = SpecializedExpert::code_generation("CodeGenerationExpert");
    let transformation = SpecializedExpert::code_transformation("CodeTransformExpert");
    let task = Task::new(TaskType::CodeTransformation, "refactor this service");
    let context = ExecutionContext::new(task.id().clone());

    let generation_output = generation.execute(&task, &context).unwrap();
    let transformation_output = transformation.execute(&task, &context).unwrap();

    assert_ne!(generation_output.content, transformation_output.content);
    assert!(transformation_output.content.contains("Transformation plan"));
}
