use crate::editor_view::EditorView;

#[test]
fn open_sets_state() {
    let mut view = EditorView::default();
    view.open("src/main.rs".to_string(), "fn main() {}".to_string());
    assert_eq!(view.open_path.as_deref(), Some("src/main.rs"));
    assert!(!view.dirty);
}

#[test]
fn edit_marks_dirty() {
    let mut view = EditorView::default();
    view.open("src/main.rs".to_string(), "fn main() {}".to_string());
    view.edit("fn main() { println!(\"hi\"); }".to_string());
    assert!(view.dirty);
}

#[test]
fn close_clears_state() {
    let mut view = EditorView::default();
    view.open("src/main.rs".to_string(), "fn main() {}".to_string());
    view.close();
    assert!(view.open_path.is_none());
    assert!(!view.dirty);
}
