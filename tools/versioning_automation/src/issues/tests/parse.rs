use crate::issues;

#[test]
fn parse_via_public_run_help_works() {
    let args = vec!["help".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_reevaluate_requires_issue() {
    let args = vec!["reevaluate".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_required_fields_validate_accepts_mode() {
    let args = vec![
        "required-fields-validate".to_string(),
        "--mode".to_string(),
        "content".to_string(),
        "--title".to_string(),
        "feat(scope): ok".to_string(),
        "--body".to_string(),
        "## Context".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_fetch_non_compliance_requires_issue_number() {
    let args = vec!["fetch-non-compliance-reason".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}
