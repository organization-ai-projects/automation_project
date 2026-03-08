use crate::engine::step_spec::StepSpec;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn task_roundtrip_json() {
    let mut metadata = BTreeMap::new();
    metadata.insert("source".to_string(), "cli".to_string());
    let mut input = BTreeMap::new();
    input.insert("prompt".to_string(), "hello".to_string());
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata,
        input,
        steps: vec![StepSpec::Log {
            message: "start".to_string(),
        }],
    };

    let json = common_json::to_string(&task).expect("serialize task");
    let restored: Task = common_json::from_json_str(&json).expect("deserialize task");
    assert_eq!(task, restored);
}
