use crate::moe_core::TaskId;

#[test]
fn task_id_new_and_as_str() {
    let id = TaskId::new("task-1");
    assert_eq!(id.as_str(), "task-1");
}
