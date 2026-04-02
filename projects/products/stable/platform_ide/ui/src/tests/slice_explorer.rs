use crate::slice_explorer::SliceExplorer;

#[test]
fn load_sets_entries() {
    let mut explorer = SliceExplorer::default();
    explorer.load(
        "issue-42".to_string(),
        vec!["src/main.rs".to_string(), "README.md".to_string()],
    );
    assert_eq!(explorer.issue_id.as_deref(), Some("issue-42"));
    assert_eq!(explorer.entries.len(), 2);
    assert!(!explorer.entries[0].dirty);
}

#[test]
fn mark_dirty_sets_flag() {
    let mut explorer = SliceExplorer::default();
    explorer.load("issue-42".to_string(), vec!["src/main.rs".to_string()]);
    explorer.mark_dirty("src/main.rs");
    assert!(explorer.entries[0].dirty);
}
