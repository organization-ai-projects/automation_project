//! tools/versioning_automation/src/automation/check_merge_conflicts.rs
use crate::automation::commands::CheckMergeConflictsOptions;
use crate::git_cli;

use super::execute::{ensure_git_repo, run_git_output};

pub(crate) fn run_check_merge_conflicts(opts: CheckMergeConflictsOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let current_branch = run_git_output(&["branch", "--show-current"])?;
    if current_branch.trim().is_empty() {
        return Err("Not on a branch (detached HEAD).".to_string());
    }

    run_git(&["fetch", "--prune", &opts.remote])?;
    if !branch_exists_remote(&opts.remote, &opts.base_branch) {
        return Err(format!(
            "Base branch '{}/{}' does not exist.",
            opts.remote, opts.base_branch
        ));
    }

    let remote_base = format!("{}/{}", opts.remote, opts.base_branch);
    let merge_base = git_cli::command(&["merge-base", "HEAD", &remote_base])
        .output()
        .map_err(|e| format!("Failed to run git merge-base HEAD {remote_base}: {e}"))?;
    if !merge_base.status.success() {
        return Err(format!(
            "Unable to compute merge base with '{remote_base}'."
        ));
    }
    let base_sha = String::from_utf8_lossy(&merge_base.stdout)
        .trim()
        .to_string();
    if base_sha.is_empty() {
        return Err("Empty merge base SHA.".to_string());
    }

    let check = git_cli::command(&["merge-tree", &base_sha, "HEAD", &remote_base])
        .output()
        .map_err(|e| format!("Failed to run git merge-tree: {e}"))?;
    if !check.status.success() {
        return Err("git merge-tree failed.".to_string());
    }
    let output = String::from_utf8_lossy(&check.stdout);
    let conflicts = output
        .lines()
        .filter_map(|line| line.strip_prefix("CONFLICT (contents): Merge conflict in "))
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if conflicts.is_empty() {
        println!("No merge conflicts detected.");
        return Ok(());
    }

    println!(
        "Merge conflicts detected for '{}' against '{}':",
        current_branch.trim(),
        remote_base
    );
    for path in conflicts {
        println!("  - {path}");
    }
    Err("Merge conflict(s) detected.".to_string())
}

fn run_git(args: &[&str]) -> Result<(), String> {
    git_cli::status(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

fn branch_exists_remote(remote: &str, branch_name: &str) -> bool {
    git_cli::status_success(&["ls-remote", "--exit-code", "--heads", remote, branch_name])
}
