//! tools/versioning_automation/src/pr/tests/closure_refs.rs
use crate::pr::commands::PrClosureRefsOptions;

#[test]
fn closure_refs_command_runs() {
    let opts = PrClosureRefsOptions {
        text: "Closes #1\nCloses rejected #2".to_string(),
    };
    let code = PrClosureRefsOptions::run_closure_refs(opts);
    assert_eq!(code, 0);
}
