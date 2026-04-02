use crate::pr::commands::pr_group_by_category_options::PrGroupByCategoryOptions;
use crate::pr::group_by_category::run_group_by_category;

#[test]
fn group_by_category_command_runs_for_resolved_mode() {
    let opts = PrGroupByCategoryOptions {
        text: "2|Bug Fixes|Closes|#2\n1|Security|Closes|#1".to_string(),
        mode: "resolved".to_string(),
    };
    let code = run_group_by_category(opts);
    assert_eq!(code, 0);
}

#[test]
fn group_by_category_command_rejects_unknown_mode() {
    let opts = PrGroupByCategoryOptions {
        text: "1|Unknown|x|y".to_string(),
        mode: "invalid".to_string(),
    };
    let code = run_group_by_category(opts);
    assert_eq!(code, 2);
}
