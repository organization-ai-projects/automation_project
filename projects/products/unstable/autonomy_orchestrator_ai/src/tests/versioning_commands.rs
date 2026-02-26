use crate::versioning::{RepoVersioningSnapshot, compute_repo_delta};

#[test]
fn compute_repo_delta_marks_changed_when_status_differs() {
    let before = RepoVersioningSnapshot {
        head_commit: Some("abc".to_string()),
        status_porcelain: " M src/main.rs\n".to_string(),
        changed_files: vec!["src/main.rs".to_string()],
    };
    let after = RepoVersioningSnapshot {
        head_commit: Some("abc".to_string()),
        status_porcelain: " M src/main.rs\n?? src/new.rs\n".to_string(),
        changed_files: vec!["src/main.rs".to_string(), "src/new.rs".to_string()],
    };

    let delta = compute_repo_delta(Some(&before), Some(&after)).expect("delta");
    assert!(delta.worktree_changed);
    assert_eq!(delta.before_head_commit.as_deref(), Some("abc"));
    assert_eq!(delta.after_head_commit.as_deref(), Some("abc"));
    assert_eq!(
        delta.touched_files,
        vec!["src/main.rs".to_string(), "src/new.rs".to_string()]
    );
}

#[test]
fn compute_repo_delta_marks_unchanged_when_status_is_identical() {
    let before = RepoVersioningSnapshot {
        head_commit: Some("abc".to_string()),
        status_porcelain: "".to_string(),
        changed_files: Vec::new(),
    };
    let after = RepoVersioningSnapshot {
        head_commit: Some("abc".to_string()),
        status_porcelain: "".to_string(),
        changed_files: Vec::new(),
    };

    let delta = compute_repo_delta(Some(&before), Some(&after)).expect("delta");
    assert!(!delta.worktree_changed);
    assert!(delta.touched_files.is_empty());
}
