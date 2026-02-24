// projects/products/unstable/platform_ide/ui/src/diff_view.rs
use serde::{Deserialize, Serialize};

/// A single diff line for display.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineKind {
    Added,
    Removed,
    Context,
}

/// A diff line for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLineEntry {
    pub kind: DiffLineKind,
    pub content: String,
}

/// The local diff view state for a single file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiffView {
    /// The file path being diffed (always an allowed path).
    pub path: Option<String>,
    /// The diff lines to display.
    pub lines: Vec<DiffLineEntry>,
}

impl DiffView {
    /// Loads the diff for a file.
    pub fn load(&mut self, path: String, lines: Vec<DiffLineEntry>) {
        self.path = Some(path);
        self.lines = lines;
    }

    /// Returns `true` if there are any added or removed lines.
    pub fn has_changes(&self) -> bool {
        self.lines.iter().any(|l| l.kind != DiffLineKind::Context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
