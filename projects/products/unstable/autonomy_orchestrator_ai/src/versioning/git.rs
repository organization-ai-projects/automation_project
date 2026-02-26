use std::path::Path;
use std::process::Command;

use crate::versioning::{RepoVersioningDelta, RepoVersioningSnapshot};

pub fn capture_repo_snapshot(repo_root: &Path) -> Option<RepoVersioningSnapshot> {
    let status_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("status")
        .arg("--porcelain")
        .arg("--untracked-files=all")
        .output()
        .ok()?;
    if !status_output.status.success() {
        return None;
    }
    let status_porcelain = String::from_utf8_lossy(&status_output.stdout).to_string();
    let changed_files = parse_changed_files_from_porcelain(&status_porcelain);

    let head_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;
    let head_commit = if head_output.status.success() {
        let head = String::from_utf8_lossy(&head_output.stdout)
            .trim()
            .to_string();
        if head.is_empty() { None } else { Some(head) }
    } else {
        None
    };

    Some(RepoVersioningSnapshot {
        head_commit,
        status_porcelain,
        changed_files,
    })
}

pub fn compute_repo_delta(
    before: Option<&RepoVersioningSnapshot>,
    after: Option<&RepoVersioningSnapshot>,
) -> Option<RepoVersioningDelta> {
    let after = after?;
    let before_head_commit = before.and_then(|snapshot| snapshot.head_commit.clone());
    let after_head_commit = after.head_commit.clone();
    let touched_files = after.changed_files.clone();
    let worktree_changed = before
        .map(|snapshot| snapshot.status_porcelain != after.status_porcelain)
        .unwrap_or(!after.status_porcelain.is_empty());

    Some(RepoVersioningDelta {
        before_head_commit,
        after_head_commit,
        touched_files,
        worktree_changed,
    })
}

fn parse_changed_files_from_porcelain(status_porcelain: &str) -> Vec<String> {
    let mut files = status_porcelain
        .lines()
        .filter_map(|line| {
            if line.len() < 4 {
                return None;
            }
            let path_part = line[3..].trim();
            if path_part.is_empty() {
                return None;
            }
            if let Some((_, rhs)) = path_part.split_once(" -> ") {
                return Some(rhs.trim().to_string());
            }
            Some(path_part.to_string())
        })
        .collect::<Vec<_>>();
    files.sort_unstable();
    files.dedup();
    files
}
