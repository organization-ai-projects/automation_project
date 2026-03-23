use crate::moe_core::TaskPriority;

#[test]
fn task_priority_variants_are_constructible() {
    let normal = TaskPriority::Normal;
    let critical = TaskPriority::Critical;
    assert!(matches!(normal, TaskPriority::Normal));
    assert!(matches!(critical, TaskPriority::Critical));
}
