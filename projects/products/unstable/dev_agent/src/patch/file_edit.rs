use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEdit {
    pub path: String,
    pub new_content: String,
}

impl FileEdit {
    #[allow(dead_code)]
    pub fn new(path: impl Into<String>, new_content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            new_content: new_content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let edit = FileEdit::new("src/main.rs", "fn main() {}");
        assert_eq!(edit.path, "src/main.rs");
        assert_eq!(edit.new_content, "fn main() {}");
    }

    #[test]
    fn serializes_roundtrip() {
        let edit = FileEdit::new("foo.rs", "hello");
        let json = serde_json::to_string(&edit).unwrap();
        let restored: FileEdit = serde_json::from_str(&json).unwrap();
        assert_eq!(edit, restored);
    }
}
