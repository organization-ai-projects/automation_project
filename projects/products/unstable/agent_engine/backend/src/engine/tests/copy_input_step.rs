use crate::engine::copy_input_step::CopyInputStep;
use crate::engine::execution_context::ExecutionContext;
use crate::engine::step::Step;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn copy_input_step_copies_value() {
    let mut input = BTreeMap::new();
    input.insert("prompt".to_string(), "hello".to_string());
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input,
        steps: vec![],
    };
    let mut ctx = ExecutionContext::new(task);
    let step = CopyInputStep {
        input_key: "prompt".to_string(),
        output_key: "echo".to_string(),
    };

    let result = step.execute(&mut ctx).expect("execute copy input step");
    assert_eq!(result.step, "copy_input");
    assert_eq!(ctx.output.get("echo"), Some(&"hello".to_string()));
}
