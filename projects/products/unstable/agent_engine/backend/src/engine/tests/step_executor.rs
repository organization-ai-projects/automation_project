use crate::engine::execution_context::ExecutionContext;
use crate::engine::log_step::LogStep;
use crate::engine::set_output_step::SetOutputStep;
use crate::engine::step::Step;
use crate::engine::step_executor::StepExecutor;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn runs_steps_in_order() {
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input: BTreeMap::new(),
        steps: vec![],
    };
    let mut ctx = ExecutionContext::new(task);
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(LogStep {
            message: "start".to_string(),
        }),
        Box::new(SetOutputStep {
            key: "status".to_string(),
            value: "ok".to_string(),
        }),
    ];

    let results = StepExecutor::run(&mut ctx, &steps).expect("run steps");
    assert_eq!(results.len(), 2);
    assert_eq!(ctx.logs, vec!["start".to_string()]);
    assert_eq!(ctx.output.get("status"), Some(&"ok".to_string()));
}
