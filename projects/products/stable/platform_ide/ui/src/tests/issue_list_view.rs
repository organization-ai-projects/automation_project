use crate::issue_entry::IssueEntry;
use crate::issue_list_view::IssueListView;

#[test]
fn set_issues_updates_list() {
    let mut view = IssueListView {
        loading: true,
        ..IssueListView::default()
    };
    view.set_issues(vec![IssueEntry {
        id: "issue-1".to_string(),
        name: "My Issue".to_string(),
        description: None,
    }]);
    assert_eq!(view.issues.len(), 1);
    assert!(!view.loading);
}
