//! tools/versioning_automation/src/pr/tests/resolve_category.rs
use crate::pr::{
    commands::{PrIssueCategoryFromLabelsOptions, PrIssueCategoryFromTitleOptions},
    resolve_category::{run_issue_category_from_labels, run_issue_category_from_title},
};

#[test]
fn issue_category_from_labels_command_runs() {
    let opts = PrIssueCategoryFromLabelsOptions {
        labels_raw: "security||bug".to_string(),
    };
    let code = run_issue_category_from_labels(opts);
    assert_eq!(code, 0);
}

#[test]
fn issue_category_from_title_command_runs() {
    let opts = PrIssueCategoryFromTitleOptions {
        title: "fix(auth): handle regression".to_string(),
    };
    let code = run_issue_category_from_title(opts);
    assert_eq!(code, 0);
}
