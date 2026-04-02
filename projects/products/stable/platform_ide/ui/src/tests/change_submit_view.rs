use crate::change_submit_view::ChangeSubmitView;

#[test]
fn stage_sets_paths() {
    let mut view = ChangeSubmitView::default();
    view.stage(
        vec!["src/main.rs".to_string()],
        "fix: update main".to_string(),
    );
    assert_eq!(view.staged_paths.len(), 1);
    assert_eq!(view.message, "fix: update main");
}

#[test]
fn on_submitted_clears_staged() {
    let mut view = ChangeSubmitView::default();
    view.stage(vec!["src/main.rs".to_string()], "msg".to_string());
    view.on_submitted("abc123".to_string());
    assert!(view.staged_paths.is_empty());
    assert_eq!(view.last_commit_id.as_deref(), Some("abc123"));
}
