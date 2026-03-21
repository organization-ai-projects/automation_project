use crate::open_pr_issue_refs::pr_body_references_issue;

#[test]
fn pr_body_references_issue_matches_effective_closes_only() {
    let body = "Closes #42\nCancel-Closes #7\nReopen #9\n";

    assert!(pr_body_references_issue(body, "#42"));
    assert!(!pr_body_references_issue(body, "#7"));
    assert!(!pr_body_references_issue(body, "#9"));
}
