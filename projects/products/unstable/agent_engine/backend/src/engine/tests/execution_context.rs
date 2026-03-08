use crate::engine::execution_context::ExecutionContext;
use crate::engine::task::Task;
use std::collections::BTreeMap;

#[test]
fn stores_logs_and_output() {
    let task = Task {
        id: "t1".to_string(),
        label: "demo".to_string(),
        metadata: BTreeMap::new(),
        input: BTreeMap::new(),
        steps: vec![],
    };
    let mut ctx = ExecutionContext::new(task);
    ctx.append_log("hello");
    ctx.set_output("k", "v");

    assert_eq!(ctx.logs, vec!["hello".to_string()]);
    assert_eq!(ctx.output.get("k"), Some(&"v".to_string()));
}
