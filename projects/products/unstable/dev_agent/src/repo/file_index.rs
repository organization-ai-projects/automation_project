use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileIndex {
    pub entries: Vec<String>,
}

impl FileIndex {
    pub fn new(entries: Vec<String>) -> Self {
        Self { entries }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_entries() {
        let idx = FileIndex::new(vec!["a.rs".to_string(), "b.rs".to_string()]);
        assert_eq!(idx.len(), 2);
        assert!(!idx.is_empty());
    }

    #[test]
    fn empty_index() {
        let idx = FileIndex::new(vec![]);
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
    }

    #[test]
    fn serializes_to_json() {
        let idx = FileIndex::new(vec!["main.rs".to_string()]);
        let json = serde_json::to_string(&idx).unwrap();
        let restored: FileIndex = serde_json::from_str(&json).unwrap();
        assert_eq!(idx, restored);
    }
}
