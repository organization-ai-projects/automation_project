//! projects/products/stable/platform_ide/backend/src/changes/tests/change_set.rs
use crate::changes::ChangeSet;
use crate::editor::FileBuffer;
use crate::slices::AllowedPath;

fn allowed(path: &str) -> AllowedPath {
    AllowedPath::new_validated(path.to_string())
}

#[test]
fn empty_change_set_is_invalid() {
    let cs = ChangeSet::new();
    assert!(cs.validate().is_err());
}

#[test]
fn clean_buffer_not_added() {
    let mut cs = ChangeSet::new();
    let buf = FileBuffer::open(allowed("a.txt"), b"hello".to_vec());
    let added = cs.add_buffer(&buf);
    assert!(!added);
    assert!(cs.is_empty());
}

#[test]
fn dirty_buffer_added() {
    let mut cs = ChangeSet::new();
    let mut buf = FileBuffer::open(allowed("a.txt"), b"hello".to_vec());
    buf.write(b"world".to_vec());
    let added = cs.add_buffer(&buf);
    assert!(added);
    assert_eq!(cs.len(), 1);
    assert!(cs.validate().is_ok());
    assert_eq!(cs.entries()[0].path.as_str(), "a.txt");
    assert_eq!(cs.entries()[0].content, b"world");
}

#[test]
fn multiple_buffers() {
    let mut cs = ChangeSet::new();
    for name in &["a.txt", "b.txt", "c.txt"] {
        let mut buf = FileBuffer::open(allowed(name), b"orig".to_vec());
        buf.write(b"new".to_vec());
        cs.add_buffer(&buf);
    }
    assert_eq!(cs.len(), 3);
}
