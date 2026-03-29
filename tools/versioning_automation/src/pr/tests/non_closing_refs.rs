//! tools/versioning_automation/src/pr/tests/non_closing_refs.rs
use crate::pr::commands::PrNonClosingRefsOptions;

#[test]
fn non_closing_refs_command_runs() {
    let opts = PrNonClosingRefsOptions {
        text: "Part of #3\nPart of #3".to_string(),
    };
    let code = PrNonClosingRefsOptions::run_non_closing_refs(opts);
    assert_eq!(code, 0);
}
