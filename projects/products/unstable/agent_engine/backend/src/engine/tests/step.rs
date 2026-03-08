use crate::engine::copy_input_step::CopyInputStep;
use crate::engine::execution_context::ExecutionContext;
use crate::engine::log_step::LogStep;
use crate::engine::set_output_step::SetOutputStep;
use crate::engine::step::Step;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn log_step_appends_log() {
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input: BTreeMap::new(),
        steps: vec![],
    };
    let mut ctx = ExecutionContext::new(task);
    let step = LogStep {
        message: "hello".to_string(),
    };
    step.execute(&mut ctx).expect("execute log step");
    assert_eq!(ctx.logs, vec!["hello".to_string()]);
}

#[test]
fn set_output_step_sets_value() {
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input: BTreeMap::new(),
        steps: vec![],
    };
    let mut ctx = ExecutionContext::new(task);
    let step = SetOutputStep {
        key: "x".to_string(),
        value: "42".to_string(),
    };
    step.execute(&mut ctx).expect("execute set output step");
    assert_eq!(ctx.output.get("x"), Some(&"42".to_string()));
}

#[test]
fn copy_input_step_copies_input() {
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
    step.execute(&mut ctx).expect("execute copy input step");
    assert_eq!(ctx.output.get("echo"), Some(&"hello".to_string()));
}
