//! projects/products/stable/platform_ide/backend/src/editor/tests/file_buffer.rs
use crate::editor::FileBuffer;
use crate::slices::AllowedPath;

fn make_path() -> AllowedPath {
    AllowedPath::new_validated("src/main.rs".to_string())
}

#[test]
fn open_not_dirty() {
    let buf = FileBuffer::open(make_path(), b"hello".to_vec());
    assert!(!buf.is_dirty());
    assert_eq!(buf.content(), b"hello");
}

#[test]
fn write_marks_dirty() {
    let mut buf = FileBuffer::open(make_path(), b"hello".to_vec());
    buf.write(b"world".to_vec());
    assert!(buf.is_dirty());
    assert_eq!(buf.content(), b"world");
    assert_eq!(buf.original(), b"hello");
}

#[test]
fn revert_clears_dirty() {
    let mut buf = FileBuffer::open(make_path(), b"hello".to_vec());
    buf.write(b"world".to_vec());
    buf.revert();
    assert!(!buf.is_dirty());
    assert_eq!(buf.content(), b"hello");
}
