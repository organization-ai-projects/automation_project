#[test]
fn join_labels_joins_non_empty_label_names() {
    let payload = crate::issues::required_fields::gh_issue_payload::GhIssuePayload {
        labels: Some(vec![
            crate::issues::required_fields::gh_issue_label::GhIssueLabel {
                name: Some("bug".to_string()),
            },
            crate::issues::required_fields::gh_issue_label::GhIssueLabel { name: None },
            crate::issues::required_fields::gh_issue_label::GhIssueLabel {
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
    let payload = crate::issues::required_fields::gh_issue_payload::GhIssuePayload {
        labels: None,
        title: None,
        body: None,
    };
    assert!(payload.join_labels().is_empty());
}
