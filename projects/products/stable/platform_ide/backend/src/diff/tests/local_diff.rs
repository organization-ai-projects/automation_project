//! projects/products/stable/platform_ide/backend/src/diff/tests/local_diff.rs
use crate::diff::{DiffLine, LocalDiff};
use crate::editor::FileBuffer;
use crate::slices::AllowedPath;

fn allowed(path: &str) -> AllowedPath {
    AllowedPath::new_validated(path.to_string())
}

#[test]
fn no_changes_produces_only_context() {
    let buf = FileBuffer::open(allowed("a.txt"), b"line1\nline2\n".to_vec());
    let diff = LocalDiff::from_buffer(&buf);
    assert!(!diff.has_changes());
    assert!(diff.lines.iter().all(|l| matches!(l, DiffLine::Context(_))));
}

#[test]
fn added_line_detected() {
    let mut buf = FileBuffer::open(allowed("a.txt"), b"line1\n".to_vec());
    buf.write(b"line1\nline2\n".to_vec());
    let diff = LocalDiff::from_buffer(&buf);
    assert!(diff.has_changes());
    assert!(diff.lines.iter().any(|l| matches!(l, DiffLine::Added(_))));
}

#[test]
fn removed_line_detected() {
    let mut buf = FileBuffer::open(allowed("a.txt"), b"line1\nline2\n".to_vec());
    buf.write(b"line1\n".to_vec());
    let diff = LocalDiff::from_buffer(&buf);
    assert!(diff.has_changes());
    assert!(diff.lines.iter().any(|l| matches!(l, DiffLine::Removed(_))));
}

#[test]
fn path_is_preserved() {
    let buf = FileBuffer::open(allowed("src/lib.rs"), b"fn foo() {}".to_vec());
    let diff = LocalDiff::from_buffer(&buf);
    assert_eq!(diff.path.as_str(), "src/lib.rs");
}
