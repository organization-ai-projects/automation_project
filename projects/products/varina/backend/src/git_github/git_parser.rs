use crate::autopilot::AutopilotError;

type Result<T> = std::result::Result<T, AutopilotError>;

/// Parses the output of `git log --oneline`.
/// Each line represents a commit in the format: `<hash> <message>`.
pub fn parse_git_log_oneline(output: &str) -> Result<Vec<(String, String)>> {
    let commits = output
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, ' ');
            let hash = parts.next().unwrap_or_default().to_string();
            let message = parts.next().unwrap_or_default().to_string();
            (hash, message)
        })
        .collect();
    Ok(commits)
}

/// Parses the output of `git diff`.
/// Extracts file paths and change types (e.g., added, modified, deleted).
pub fn parse_git_diff(output: &str) -> Result<Vec<(String, String)>> {
    let changes = output
        .lines()
        .filter_map(|line| {
            if line.starts_with("diff --git") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    Some((parts[1].to_string(), parts[2].to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    Ok(changes)
}

/// Parses the output of `git branch`.
/// Extracts branch names and identifies the current branch.
pub fn parse_git_branch(output: &str) -> Result<(String, Vec<String>)> {
    let mut branches = Vec::new();
    let mut current_branch = String::new();

    for line in output.lines() {
        if line.starts_with("*") {
            current_branch = line[2..].to_string();
        } else {
            branches.push(line.trim().to_string());
        }
    }

    Ok((current_branch, branches))
}

/// Parses the output of `git show`.
/// Extracts commit details like hash, author, date, and message.
pub fn parse_git_show(output: &str) -> Result<(String, String, String, String)> {
    let mut lines = output.lines();
    let hash = lines.next().unwrap_or_default().to_string();
    let author = lines.next().unwrap_or_default().to_string();
    let date = lines.next().unwrap_or_default().to_string();
    let message = lines.collect::<Vec<&str>>().join("\n");

    Ok((hash, author, date, message))
}
