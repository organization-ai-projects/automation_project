// projects/products/unstable/platform_ide/backend/src/diff/local_diff.rs
use crate::editor::FileBuffer;
use crate::slices::AllowedPath;

/// A single changed line in a local diff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    /// A line present only in the original.
    Removed(String),
    /// A line present only in the modified content.
    Added(String),
    /// A line unchanged between original and modified.
    Context(String),
}

/// A local diff between the original and current content of a file buffer.
///
/// Only the path that was validated through the slice manifest is stored;
/// no forbidden paths can appear in a `LocalDiff`.
#[derive(Debug)]
pub struct LocalDiff {
    /// The validated path of the diffed file.
    pub path: AllowedPath,
    /// The diff lines (context, added, removed).
    pub lines: Vec<DiffLine>,
}

impl LocalDiff {
    /// Computes a line-by-line diff from a `FileBuffer`.
    pub fn from_buffer(buf: &FileBuffer) -> Self {
        let original_text = String::from_utf8_lossy(buf.original());
        let current_text = String::from_utf8_lossy(buf.content());

        let original_lines: Vec<&str> = original_text.lines().collect();
        let current_lines: Vec<&str> = current_text.lines().collect();

        let lines = compute_diff_lines(&original_lines, &current_lines);

        Self {
            path: buf.path.clone(),
            lines,
        }
    }

    /// Returns `true` if the diff contains any additions or removals.
    pub fn has_changes(&self) -> bool {
        self.lines
            .iter()
            .any(|l| matches!(l, DiffLine::Added(_) | DiffLine::Removed(_)))
    }
}

fn compute_diff_lines(original: &[&str], current: &[&str]) -> Vec<DiffLine> {
    let lcs = lcs_table(original, current);
    let mut result = Vec::new();
    backtrack(
        &lcs,
        original,
        current,
        original.len(),
        current.len(),
        &mut result,
    );
    result
}

fn lcs_table(a: &[&str], b: &[&str]) -> Vec<Vec<usize>> {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp
}

fn backtrack(
    dp: &[Vec<usize>],
    a: &[&str],
    b: &[&str],
    i: usize,
    j: usize,
    out: &mut Vec<DiffLine>,
) {
    if i == 0 && j == 0 {
        return;
    }
    if i == 0 {
        backtrack(dp, a, b, i, j - 1, out);
        out.push(DiffLine::Added(b[j - 1].to_string()));
    } else if j == 0 {
        backtrack(dp, a, b, i - 1, j, out);
        out.push(DiffLine::Removed(a[i - 1].to_string()));
    } else if a[i - 1] == b[j - 1] {
        backtrack(dp, a, b, i - 1, j - 1, out);
        out.push(DiffLine::Context(a[i - 1].to_string()));
    } else if dp[i - 1][j] >= dp[i][j - 1] {
        backtrack(dp, a, b, i - 1, j, out);
        out.push(DiffLine::Removed(a[i - 1].to_string()));
    } else {
        backtrack(dp, a, b, i, j - 1, out);
        out.push(DiffLine::Added(b[j - 1].to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::FileBuffer;
    use crate::slices::AllowedPath;

    fn allowed(path: &str) -> AllowedPath {
        AllowedPath::new_validated(path.to_string())
    }

    #[test]
    fn no_changes_produces_only_context() {
        let buf = FileBuffer::open(allowed("a.txt"), b"line1\nline2\n".to_vec());
        let diff = LocalDiff::from_buffer(&buf);
        assert!(!diff.has_changes());
        assert!(diff.lines.iter().all(|l| matches!(l, DiffLine::Context(_))));
    }

    #[test]
    fn added_line_detected() {
        let mut buf = FileBuffer::open(allowed("a.txt"), b"line1\n".to_vec());
        buf.write(b"line1\nline2\n".to_vec());
        let diff = LocalDiff::from_buffer(&buf);
        assert!(diff.has_changes());
        assert!(diff.lines.iter().any(|l| matches!(l, DiffLine::Added(_))));
    }

    #[test]
    fn removed_line_detected() {
        let mut buf = FileBuffer::open(allowed("a.txt"), b"line1\nline2\n".to_vec());
        buf.write(b"line1\n".to_vec());
        let diff = LocalDiff::from_buffer(&buf);
        assert!(diff.has_changes());
        assert!(diff.lines.iter().any(|l| matches!(l, DiffLine::Removed(_))));
    }

    #[test]
    fn path_is_preserved() {
        let buf = FileBuffer::open(allowed("src/lib.rs"), b"fn foo() {}".to_vec());
        let diff = LocalDiff::from_buffer(&buf);
        assert_eq!(diff.path.as_str(), "src/lib.rs");
    }
}
