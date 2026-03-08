use crate::engine::step_spec::StepSpec;
use crate::engine::task::Task;
use crate::engine::task_runner::run_task;
use std::collections::BTreeMap;

#[test]
fn run_task_produces_outcome() {
    let mut input = BTreeMap::new();
    input.insert("prompt".to_string(), "hello".to_string());

    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input,
        steps: vec![
            StepSpec::Log {
                message: "begin".to_string(),
            },
            StepSpec::CopyInput {
                input_key: "prompt".to_string(),
                output_key: "echo".to_string(),
            },
            StepSpec::SetOutput {
                key: "status".to_string(),
                value: "ok".to_string(),
            },
        ],
    };

    let out = run_task(task).expect("run task");
    assert!(out.success);
    assert_eq!(out.logs, vec!["begin".to_string()]);
    assert_eq!(out.output.get("echo"), Some(&"hello".to_string()));
    assert_eq!(out.output.get("status"), Some(&"ok".to_string()));
}
