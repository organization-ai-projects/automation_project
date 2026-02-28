use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn walk_files_sorted(root: &str, skip_dirs: &[String]) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    collect_files(Path::new(root), skip_dirs, &mut result)?;
    result.sort();
    Ok(result)
}

fn collect_files(dir: &Path, skip_dirs: &[String], result: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let mut entries: Vec<_> = std::fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if path.is_dir() {
            if skip_dirs.iter().any(|s| s == name_str.as_ref()) {
                continue;
            }
            collect_files(&path, skip_dirs, result)?;
        } else {
            result.push(path);
        }
    }
    Ok(())
}
