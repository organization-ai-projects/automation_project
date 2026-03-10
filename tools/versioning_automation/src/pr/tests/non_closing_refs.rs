use crate::pr::commands::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::non_closing_refs::run_non_closing_refs;

#[test]
fn non_closing_refs_command_runs() {
    let opts = PrNonClosingRefsOptions {
        text: "Part of #3\nPart of #3".to_string(),
    };
    let code = run_non_closing_refs(opts);
    assert_eq!(code, 0);
}
