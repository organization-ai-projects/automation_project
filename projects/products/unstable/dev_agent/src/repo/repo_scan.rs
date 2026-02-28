use crate::diagnostics::error::AgentError;
use crate::repo::file_index::FileIndex;
use crate::repo::repo_root::RepoRoot;
use std::path::Path;

pub struct RepoScan;

impl RepoScan {
    pub fn new() -> Self {
        Self
    }

    /// Scans the given repo root and returns a deterministic `FileIndex`.
    /// Entries are relative paths sorted lexicographically.
    pub fn scan(&self, root: &RepoRoot) -> Result<FileIndex, AgentError> {
        let mut entries = collect_files(&root.path, &root.path)?;
        entries.sort();
        Ok(FileIndex::new(entries))
    }
}

impl Default for RepoScan {
    fn default() -> Self {
        Self::new()
    }
}

fn collect_files(root: &Path, dir: &Path) -> Result<Vec<String>, AgentError> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| AgentError::Io(e.to_string()))? {
        let entry = entry.map_err(|e| AgentError::Io(e.to_string()))?;
        let path = entry.path();
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if file_name.starts_with('.') || file_name == "target" {
            continue;
        }
        if path.is_dir() {
            let sub = collect_files(root, &path)?;
            result.extend(sub);
        } else if path.is_file() {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            result.push(rel);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn temp_dir(suffix: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let pid = std::process::id();
        let dir = std::env::temp_dir().join(format!("dev_agent_scan_{suffix}_{pid}_{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn scan_returns_sorted_entries() {
        let dir = temp_dir("sorted");
        fs::write(dir.join("b.rs"), "").unwrap();
        fs::write(dir.join("a.rs"), "").unwrap();
        let root = RepoRoot::new(dir.clone());
        let idx = RepoScan::new().scan(&root).unwrap();
        let mut expected = idx.entries.clone();
        expected.sort();
        assert_eq!(idx.entries, expected);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn scan_is_deterministic() {
        let dir = temp_dir("det");
        fs::write(dir.join("foo.rs"), "x").unwrap();
        fs::write(dir.join("bar.rs"), "y").unwrap();
        let root = RepoRoot::new(dir.clone());
        let idx1 = RepoScan::new().scan(&root).unwrap();
        let idx2 = RepoScan::new().scan(&root).unwrap();
        assert_eq!(idx1, idx2);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn scan_skips_hidden_and_target() {
        let dir = temp_dir("skip");
        fs::write(dir.join("visible.rs"), "").unwrap();
        fs::create_dir(dir.join(".hidden")).unwrap();
        let mut f = std::fs::File::create(dir.join(".hidden").join("secret.rs")).unwrap();
        write!(f, "").unwrap();
        fs::create_dir(dir.join("target")).unwrap();
        fs::write(dir.join("target").join("build.rs"), "").unwrap();
        let root = RepoRoot::new(dir.clone());
        let idx = RepoScan::new().scan(&root).unwrap();
        assert_eq!(idx.entries, vec!["visible.rs"]);
        fs::remove_dir_all(dir).unwrap();
    }
}
