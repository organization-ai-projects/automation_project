/// Utility functions for goal-related operations.
use crate::ids::IssueNumber;

/// Extracts an issue number from a goal string.
pub fn extract_issue_number_from_goal(goal: &str) -> Option<IssueNumber> {
    let bytes = goal.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        if bytes[i] == b'#' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start {
                let raw = goal[start..end].parse::<u64>().ok()?;
                return IssueNumber::new(raw);
            }
        }
        i += 1;
    }
    None
}
