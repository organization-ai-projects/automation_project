use std::collections::BTreeMap;

use crate::diagnostics::error::Error;
use crate::index::doc_id::DocId;
use crate::index::inverted_index::InvertedIndex;
use crate::tokenize::tokenizer::Tokenizer;

/// Builds the inverted index from a set of documents.
pub(crate) struct IndexStore;

impl IndexStore {
    /// Index all .txt and .md files under the given root directory.
    pub(crate) fn build_from_dir(root: &std::path::Path) -> Result<InvertedIndex, Error> {
        let mut index = InvertedIndex::new();
        let mut files: BTreeMap<String, String> = BTreeMap::new();

        Self::collect_files(root, root, &mut files)?;

        for (rel_path, content) in &files {
            let doc_id = DocId::from_path(rel_path);
            let tokens = Tokenizer::tokenize(content);
            index.add_document(&doc_id, &tokens);
        }

        Ok(index)
    }

    fn collect_files(
        root: &std::path::Path,
        dir: &std::path::Path,
        files: &mut BTreeMap<String, String>,
    ) -> Result<(), Error> {
        let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(dir)
            .map_err(|e| Error::Io(e.to_string()))?
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                Self::collect_files(root, &path, files)?;
            } else if Self::is_indexable(&path) {
                let content =
                    std::fs::read_to_string(&path).map_err(|e| Error::Io(e.to_string()))?;
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(rel, content);
            }
        }
        Ok(())
    }

    fn is_indexable(path: &std::path::Path) -> bool {
        matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("txt" | "md" | "rs" | "toml" | "json" | "yaml" | "yml" | "log")
        )
    }
}
