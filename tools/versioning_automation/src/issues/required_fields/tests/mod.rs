mod contract_values;
mod gh_issue_payload;
mod key;
mod validation;

use crate::issues::required_fields::{
    non_compliance_reason_from_content, validate_content, validate_title,
};

#[test]
fn validate_title_accepts_default_conventional_pattern() {
    let validations = validate_title("feat(scope): summary", "").expect("title validation");
    assert!(validations.is_empty());
}

#[test]
fn validate_content_reports_missing_parent_field() {
    let body = "## Context\n\nctx\n\n## Problem\n\nproblem\n\n## Acceptance Criteria\n\nDone when :\n\n- [ ] ok\n\n## Hierarchy\n\n";
    let validations =
        validate_content("feat(scope): summary", body, "").expect("content validation");

    assert!(
        validations
            .iter()
            .any(|entry| entry.code == "missing_field" && entry.field == "Parent")
    );
}

#[test]
fn non_compliance_reason_respects_required_missing_label() {
    let reason = non_compliance_reason_from_content(
        "feat(scope): summary",
        "",
        "foo||issue-required-missing||bar",
    )
    .expect("reason");

    assert_eq!(reason, "label issue-required-missing is set on issue");
}
