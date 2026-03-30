//! tools/versioning_automation/src/issues/required_fields/tests/gh_issue_payload.rs
use crate::issues::required_fields::{GhIssuePayload, gh_issue_label::GhIssueLabel};

#[test]
fn join_labels_joins_non_empty_label_names() {
    let payload = GhIssuePayload {
        labels: Some(vec![
            GhIssueLabel {
                name: Some("bug".to_string()),
            },
            GhIssueLabel { name: None },
            GhIssueLabel {
                name: Some("priority-high".to_string()),
            },
        ]),
        title: None,
        body: None,
    };

    assert_eq!(payload.join_labels(), "bug||priority-high");
}

#[test]
fn join_labels_returns_empty_when_labels_absent() {
    let payload = GhIssuePayload {
        labels: None,
        title: None,
        body: None,
    };
    assert!(payload.join_labels().is_empty());
}
