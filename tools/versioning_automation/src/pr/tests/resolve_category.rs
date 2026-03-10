use crate::pr::commands::pr_effective_category_options::PrEffectiveCategoryOptions;
use crate::pr::commands::pr_issue_category_from_labels_options::PrIssueCategoryFromLabelsOptions;
use crate::pr::commands::pr_issue_category_from_title_options::PrIssueCategoryFromTitleOptions;
use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;
use crate::pr::resolve_category::{
    run_effective_category, run_issue_category_from_labels, run_issue_category_from_title,
    run_resolve_category,
};

#[test]
fn resolve_category_command_runs() {
    let opts = PrResolveCategoryOptions {
        label_category: "Unknown".to_string(),
        title_category: "UI".to_string(),
        default_category: "Mixed".to_string(),
    };
    let code = run_resolve_category(opts);
    assert_eq!(code, 0);
}

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

#[test]
fn effective_category_command_runs() {
    let opts = PrEffectiveCategoryOptions {
        labels_raw: "feature||automation".to_string(),
        title: Some("feat(ui): add dashboard".to_string()),
        title_category: None,
        default_category: "Mixed".to_string(),
    };
    let code = run_effective_category(opts);
    assert_eq!(code, 0);
}
