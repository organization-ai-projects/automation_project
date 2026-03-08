use crate::engine::execution_context::ExecutionContext;
use crate::engine::log_step::LogStep;
use crate::engine::step::Step;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn log_step_emits_log_artifact() {
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

    let result = step.execute(&mut ctx).expect("execute log step");
    assert_eq!(result.step, "log");
    assert_eq!(ctx.logs, vec!["hello".to_string()]);
}
