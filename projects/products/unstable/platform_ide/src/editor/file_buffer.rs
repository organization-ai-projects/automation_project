// projects/products/unstable/platform_ide/src/editor/file_buffer.rs
use crate::slices::AllowedPath;

/// An in-memory buffer for a file open in the IDE editor.
///
/// A `FileBuffer` can only be created for a path that has been validated
/// through a [`crate::slices::SliceManifest`]. The original content is
/// retained so that local diffs can be computed without a round-trip to the
/// platform.
#[derive(Debug)]
pub struct FileBuffer {
    /// The validated path of the open file.
    pub path: AllowedPath,
    /// The content as it was fetched from the platform.
    original: Vec<u8>,
    /// The current (possibly modified) content.
    current: Vec<u8>,
}

impl FileBuffer {
    /// Creates a new buffer for the given allowed path and initial content.
    pub fn open(path: AllowedPath, content: Vec<u8>) -> Self {
        Self {
            path,
            original: content.clone(),
            current: content,
        }
    }

    /// Returns the current content of the buffer.
    pub fn content(&self) -> &[u8] {
        &self.current
    }

    /// Returns the original content as fetched from the platform.
    pub fn original(&self) -> &[u8] {
        &self.original
    }

    /// Returns `true` if the buffer has been modified since it was opened.
    pub fn is_dirty(&self) -> bool {
        self.current != self.original
    }

    /// Replaces the entire buffer content with `new_content`.
    pub fn write(&mut self, new_content: Vec<u8>) {
        self.current = new_content;
    }

    /// Resets the buffer to the original content, discarding any edits.
    pub fn revert(&mut self) {
        self.current = self.original.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
