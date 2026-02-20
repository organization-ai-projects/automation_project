// projects/libraries/common_parsing/src/diff_parser.rs
use std::collections::HashSet;
use std::path::PathBuf;

/// Parses unified diff (patch) text and returns unique touched path *tokens* as Strings.
/// This does NOT compute diffs, it only parses diff text.
pub fn parse_unified_diff_touched_path_strings(unified_diff: &str) -> Vec<String> {
    let mut seen = HashSet::<String>::new();
    let mut out = Vec::<String>::new();

    for p in unified_diff
        .lines()
        .filter_map(|line| line.strip_prefix("+++ b/"))
        .map(str::trim)
        .filter(|p| !p.is_empty() && *p != "/dev/null")
    {
        // preserve first-seen order
        if seen.insert(p.to_string()) {
            out.push(p.to_string());
        }
    }

    out
}

/// Parses unified diff (patch) text and returns touched paths as PathBuf.
/// Note: returned paths are NOT validated against the filesystem.
pub fn parse_unified_diff_touched_paths(unified_diff: &str) -> Vec<PathBuf> {
    parse_unified_diff_touched_path_strings(unified_diff)
        .into_iter()
        .map(PathBuf::from)
        .collect()
}
