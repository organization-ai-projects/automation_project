//! tools/versioning_automation/src/pr/tests/directives_apply.rs
use crate::pr::commands::PrDirectivesApplyOptions;

#[test]
fn directives_apply_runs_with_close_and_reopen() {
    let opts = PrDirectivesApplyOptions {
        text: "Closes #12\nReopen #12\nDuplicate #4 with #12".to_string(),
    };

    let code = PrDirectivesApplyOptions::run_directives_apply(opts);
    assert_eq!(code, 0);
}
