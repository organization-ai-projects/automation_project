use crate::diff_line_entry::DiffLineEntry;
use crate::diff_line_kind::DiffLineKind;
use crate::diff_view::DiffView;

#[test]
fn load_sets_fields() {
    let mut view = DiffView::default();
    view.load(
        "src/main.rs".to_string(),
        vec![
            DiffLineEntry {
                kind: DiffLineKind::Context,
                content: "fn main() {}".to_string(),
            },
            DiffLineEntry {
                kind: DiffLineKind::Added,
                content: "println!(\"hello\");".to_string(),
            },
        ],
    );
    assert_eq!(view.path.as_deref(), Some("src/main.rs"));
    assert_eq!(view.lines.len(), 2);
    assert!(view.has_changes());
}

#[test]
fn no_changes_when_all_context() {
    let mut view = DiffView::default();
    view.load(
        "a.txt".to_string(),
        vec![DiffLineEntry {
            kind: DiffLineKind::Context,
            content: "unchanged".to_string(),
        }],
    );
    assert!(!view.has_changes());
}
