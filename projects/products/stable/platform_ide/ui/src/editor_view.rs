// projects/products/stable/platform_ide/ui/src/editor_view.rs
use serde::{Deserialize, Serialize};

/// The editor view state for a single open file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EditorView {
    /// The currently open file path (always an allowed path).
    pub open_path: Option<String>,
    /// The current content of the editor buffer as a UTF-8 string.
    pub content: Option<String>,
    /// Whether the buffer has unsaved changes.
    pub dirty: bool,
}

impl EditorView {
    /// Opens a file with the given content.
    pub fn open(&mut self, path: String, content: String) {
        self.open_path = Some(path);
        self.content = Some(content);
        self.dirty = false;
    }

    /// Updates the buffer content and marks it dirty.
    pub fn edit(&mut self, new_content: String) {
        self.content = Some(new_content);
        self.dirty = true;
    }

    /// Closes the editor.
    pub fn close(&mut self) {
        self.open_path = None;
        self.content = None;
        self.dirty = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
