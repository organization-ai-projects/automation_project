use crate::engine::execution_context::ExecutionContext;
use crate::engine::set_output_step::SetOutputStep;
use crate::engine::step::Step;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn set_output_step_stores_value() {
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input: BTreeMap::new(),
        steps: vec![],
    };
    let mut ctx = ExecutionContext::new(task);
    let step = SetOutputStep {
        key: "status".to_string(),
        value: "ok".to_string(),
    };

    let result = step.execute(&mut ctx).expect("execute set output step");
    assert_eq!(result.step, "set_output");
    assert_eq!(ctx.output.get("status"), Some(&"ok".to_string()));
}
