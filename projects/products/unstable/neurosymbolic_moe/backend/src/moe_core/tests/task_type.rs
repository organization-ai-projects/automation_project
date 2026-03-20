use crate::moe_core::TaskType;

#[test]
fn task_type_variants_are_constructible() {
    let routing = TaskType::Retrieval;
    let custom = TaskType::Custom("x".to_string());
    assert!(matches!(routing, TaskType::Retrieval));
    assert!(matches!(custom, TaskType::Custom(_)));
}
