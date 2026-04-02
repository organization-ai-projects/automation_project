//! tools/versioning_automation/src/pr/tests/issue_outcomes_snapshot.rs
use crate::pr::issue_outcomes_snapshot::build_issue_outcomes_snapshot;

#[test]
fn build_issue_outcomes_snapshot_splits_conflicts_and_effective_actions() {
    let snapshot = build_issue_outcomes_snapshot(
        &[
            crate::pr::commit_info::CommitInfo {
                short_hash: "1".to_string(),
                subject: "feat(test): reopen simple issue".to_string(),
                body: "Closes #12\nCancel-Closes #12\nReopen #12".to_string(),
            },
            crate::pr::commit_info::CommitInfo {
                short_hash: "2".to_string(),
                subject: "fix(test): close issue".to_string(),
                body: "Closes #34".to_string(),
            },
            crate::pr::commit_info::CommitInfo {
                short_hash: "3".to_string(),
                subject: "feat(test): resolve conflict".to_string(),
                body: "Closes #56\nReopen #56".to_string(),
            },
        ],
        |_, default_category| default_category.to_string(),
    );

    assert!(snapshot.close_only.iter().any(|entry| entry.0 == "#34"));
    assert!(snapshot.reopen_only.iter().any(|entry| entry.0 == "#12"));
    assert_eq!(snapshot.resolved_conflicts.len(), 1);
    assert_eq!(snapshot.resolved_conflicts[0].0, "#56");
    assert_eq!(snapshot.resolved_conflicts[0].1, "Features");
    assert_eq!(snapshot.resolved_conflicts[0].2, "reopen");
    assert!(snapshot.unresolved_conflicts.is_empty());
}
