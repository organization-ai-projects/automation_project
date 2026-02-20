// projects/libraries/common/src/string_manipulate.rs
use std::path::{Path, PathBuf};

/// projects/libraries/common/src/string_utils.rs
/// Converts a byte slice to a trimmed UTF-8 string.
pub fn trim_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

/// UTF-8 safe truncation (never panics).
pub fn truncate_utf8(mut s: String, max_chars_approx: usize) -> String {
    // This uses bytes length as a quick gate; then truncates on char boundary.
    if s.len() <= max_chars_approx {
        return s;
    }
    let mut cut = 0usize;
    for (i, _) in s.char_indices() {
        if i > max_chars_approx {
            break;
        }
        cut = i;
    }
    s.truncate(cut);
    s
}

/// Normalizes a repository path. Defaults to `.` if the path is empty.
pub fn normalize_repo_path(repo_path: &Path) -> PathBuf {
    if repo_path.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        repo_path.to_path_buf()
    }
}
