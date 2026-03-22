use crate::echo_expert::EchoExpert;
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertStatus, ExpertType, Task, TaskType,
};
use crate::orchestrator::Version;
use protocol::ProtocolId;
use std::str::FromStr;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

#[test]
fn echo_expert_new_sets_metadata_correctly() {
    let capabilities = vec![ExpertCapability::Routing];
    let expert = EchoExpert::new_with_id(protocol_id(1), "echo", capabilities.clone());

    let metadata = expert.metadata();

    assert_eq!(metadata.name, "echo");
    assert_eq!(metadata.version, Version::new(1, 0, 0));
    assert_eq!(metadata.capabilities, capabilities);
    assert_eq!(metadata.status, ExpertStatus::Active);
    assert_eq!(metadata.expert_type, ExpertType::Deterministic);
}

#[test]
fn echo_expert_id_matches_metadata_id() {
    let expert = EchoExpert::new_with_id(protocol_id(1), "echo", vec![ExpertCapability::Routing]);

    assert_eq!(expert.id(), &expert.metadata().id);
}

#[test]
fn echo_expert_can_handle_returns_false_when_task_input_is_empty() {
    let expert = EchoExpert::new_with_id(protocol_id(1), "echo", vec![ExpertCapability::Routing]);

    let task = Task::new(TaskType::Custom("default".to_string()), String::new());

    assert!(!expert.can_handle(&task));
}

#[test]
fn echo_expert_can_handle_returns_true_when_task_input_is_not_empty() {
    let expert = EchoExpert::new_with_id(protocol_id(1), "echo", vec![ExpertCapability::Routing]);

    let task = Task::new(TaskType::Custom("default".to_string()), "hello".to_string());

    assert!(expert.can_handle(&task));
}

#[test]
fn echo_expert_execute_returns_real_expected_output() {
    let expert = EchoExpert::new_with_id(protocol_id(1), "echo", vec![ExpertCapability::Routing]);

    let task = Task::new(TaskType::Custom("default".to_string()), "hello".to_string());

    let context = ExecutionContext::new(task.id().clone())
        .with_retrieved_context(vec!["ctx1".to_string(), "ctx2".to_string()])
        .with_memory_entries(vec!["mem1".to_string()])
        .with_buffer_data(vec![
            "buf1".to_string(),
            "buf2".to_string(),
            "buf3".to_string(),
        ]);

    let output = expert.execute(&task, &context).unwrap();

    assert_eq!(output.expert_id, expert.metadata().id);
    assert_eq!(
        output.content,
        "[echo] processed: hello (ctx:2 mem:1 buf:3)"
    );
    assert_eq!(output.confidence, 0.9);
    assert!(output.metadata.is_empty());
    assert_eq!(output.trace, vec!["Expert echo executed".to_string()]);
}
