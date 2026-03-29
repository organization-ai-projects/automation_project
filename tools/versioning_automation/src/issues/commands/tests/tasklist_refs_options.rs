//! tools/versioning_automation/src/issues/commands/tests/tasklist_refs_options.rs
use crate::issues::commands::tasklist_refs_options::TasklistRefsOptions;

#[test]
fn test_run_tasklist_refs() {
    let options = TasklistRefsOptions {
        body: "- [ ] Task 1\n- [x] Task 2".to_string(),
    };
    let result = options.run_tasklist_refs();
    assert_eq!(result, 0);
}
