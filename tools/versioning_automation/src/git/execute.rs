//! tools/versioning_automation/src/git/execute.rs
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::git::commands::{
    AddCommitPushOptions, BranchCreationCheckOptions, CleanBranchesOptions, CleanLocalGoneOptions,
    CleanupAfterPrOptions, CreateAfterDeleteOptions, CreateBranchOptions, CreateWorkBranchOptions,
    DeleteBranchOptions, FinishBranchOptions, GitAction, PushBranchOptions,
};
use crate::git::parse::parse;
use crate::git::render::print_usage;
use crate::git_cli;
use crate::lazy_regex::COMMIT_MESSAGE_FORMAT_REGEX;

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(action) => run_action(action),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_action(action: GitAction) -> i32 {
    let result = match action {
        GitAction::Help => {
            print_usage();
            Ok(())
        }
        GitAction::CreateBranch(opts) => run_create_branch(opts),
        GitAction::CreateWorkBranch(opts) => run_create_work_branch(opts),
        GitAction::PushBranch(opts) => run_push_branch(opts),
        GitAction::AddCommitPush(opts) => run_add_commit_push(opts),
        GitAction::DeleteBranch(opts) => run_delete_branch(opts),
        GitAction::FinishBranch(opts) => run_finish_branch(opts),
        GitAction::CreateAfterDelete(opts) => run_create_after_delete(opts),
        GitAction::CleanLocalGone(opts) => run_clean_local_gone(opts),
        GitAction::CleanBranches(opts) => run_clean_branches(opts),
        GitAction::CleanupAfterPr(opts) => run_cleanup_after_pr(opts),
        GitAction::BranchCreationCheck(opts) => run_branch_creation_check(opts),
    };

    match result {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn run_branch_creation_check(opts: BranchCreationCheckOptions) -> Result<(), String> {
    let Some(command) = opts.command else {
        return run_git_passthrough(&[]);
    };

    if command != "branch" && command != "checkout" && command != "switch" {
        let mut passthrough = Vec::with_capacity(1 + opts.args.len());
        passthrough.push(command);
        passthrough.extend(opts.args);
        let refs = passthrough.iter().map(String::as_str).collect::<Vec<_>>();
        return run_git_passthrough(&refs);
    }

    let mut branch_to_check: Option<String> = None;
    let mut i = 0usize;
    while i < opts.args.len() {
        let arg = opts.args[i].as_str();
        match arg {
            "-b" | "-c" | "--branch" | "--create" | "-B" | "-C" | "--force-create" => {
                if i + 1 < opts.args.len() {
                    branch_to_check = Some(opts.args[i + 1].clone());
                    i += 2;
                    continue;
                }
            }
            _ => {
                if command == "branch" && !arg.starts_with('-') && branch_to_check.is_none() {
                    branch_to_check = Some(arg.to_string());
                }
            }
        }
        i += 1;
    }

    if let Some(branch) = branch_to_check {
        let marker = format!("[{branch}]");
        let worktrees = run_git_output(&["worktree", "list"])?;
        if worktrees.lines().any(|line| line.contains(&marker)) {
            eprintln!(
                "❌ The branch '{}' is already in use by another worktree:",
                branch
            );
            for line in worktrees.lines().filter(|line| line.contains(&marker)) {
                eprintln!("{line}");
            }
            eprintln!("   Remove it with: git worktree remove <path>");
            return Err("branch already attached to another worktree".to_string());
        }
    }

    let mut passthrough = Vec::with_capacity(1 + opts.args.len());
    passthrough.push(command);
    passthrough.extend(opts.args);
    let refs = passthrough.iter().map(String::as_str).collect::<Vec<_>>();
    run_git_passthrough(&refs)
}

fn run_git_passthrough(args: &[&str]) -> Result<(), String> {
    let mut cmd = Command::new("git");
    cmd.args(args);
    let status = cmd
        .status()
        .map_err(|err| format!("failed to execute git {}: {}", args.join(" "), err))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "git {} failed with exit status {:?}",
            args.join(" "),
            status.code()
        ))
    }
}

fn run_create_branch(opts: CreateBranchOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let mut branch_name = opts
        .branch_name
        .or_else(load_last_deleted_branch)
        .ok_or_else(|| "Missing branch name and no last deleted branch found.".to_string())?;
    branch_name = branch_name.trim().to_string();

    validate_branch_name(&branch_name)?;
    require_non_protected_branch(&branch_name)?;

    git_fetch_prune(&opts.remote)?;
    run_git(&["checkout", &opts.base_branch])?;
    run_git(&["pull", &opts.remote, &opts.base_branch])?;

    if branch_exists_local(&branch_name) {
        run_git(&["checkout", &branch_name])?;
    } else {
        run_git(&["checkout", "-b", &branch_name, &opts.base_branch])?;
    }

    run_git(&["push", "--set-upstream", &opts.remote, &branch_name])?;
    Ok(())
}

fn run_create_work_branch(opts: CreateWorkBranchOptions) -> Result<(), String> {
    ensure_git_repo()?;
    validate_branch_type(&opts.branch_type)?;
    require_clean_tree()?;

    let description = sanitize_description(&opts.description);
    if description.is_empty() {
        return Err("Description cannot be empty after sanitization.".to_string());
    }

    let branch_name = format!("{}/{}", opts.branch_type, description);
    validate_branch_name(&branch_name)?;

    git_fetch_prune(&opts.remote)?;
    if branch_exists_local(&branch_name) {
        return Err(format!("Branch '{branch_name}' already exists locally."));
    }
    if branch_exists_remote(&opts.remote, &branch_name) {
        return Err(format!(
            "Branch '{branch_name}' already exists on remote {}.",
            opts.remote
        ));
    }

    if branch_exists_local(&opts.base_branch) {
        run_git(&["checkout", &opts.base_branch])?;
    } else {
        let remote_ref = format!("{}/{}", opts.remote, opts.base_branch);
        run_git(&["checkout", "-b", &opts.base_branch, &remote_ref])?;
    }

    run_git(&["pull", &opts.remote, &opts.base_branch])?;
    run_git(&["checkout", "-b", &branch_name, &opts.base_branch])?;
    run_git(&["push", "--set-upstream", &opts.remote, &branch_name])?;
    Ok(())
}

fn run_push_branch(opts: PushBranchOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let branch_name = current_branch()?;
    require_non_protected_branch(&branch_name)?;

    git_fetch_prune(&opts.remote)?;

    if has_upstream() {
        run_git(&["push", &opts.remote, &branch_name])?;
    } else {
        run_git(&["push", "--set-upstream", &opts.remote, &branch_name])?;
    }

    Ok(())
}

fn run_add_commit_push(opts: AddCommitPushOptions) -> Result<(), String> {
    ensure_git_repo()?;
    validate_commit_message(&opts.message)?;

    let branch_name = current_branch()?;
    require_non_protected_branch(&branch_name)?;

    run_git(&["add", "-A"])?;

    let staged_files = run_git_output(&["diff", "--cached", "--name-only"])?;
    if staged_files.trim().is_empty() {
        return Ok(());
    }

    if opts.no_verify {
        run_git(&["commit", "--no-verify", "-m", &opts.message])?;
    } else {
        run_git(&["commit", "-m", &opts.message])?;
    }

    run_push_branch(PushBranchOptions {
        remote: opts.remote,
    })
}

fn run_delete_branch(opts: DeleteBranchOptions) -> Result<(), String> {
    ensure_git_repo()?;

    let branch_name = opts.branch_name.trim();
    if branch_name.is_empty() {
        return Err("Branch name cannot be empty.".to_string());
    }
    require_non_protected_branch(branch_name)?;

    let current = current_branch()?;
    if current == branch_name {
        run_git(&["checkout", &opts.base_branch])?;
    }

    save_last_deleted_branch(branch_name)?;
    git_fetch_prune(&opts.remote)?;

    if branch_exists_local(branch_name) {
        if opts.force {
            run_git(&["branch", "-D", branch_name])?;
        } else {
            run_git(&["branch", "-d", branch_name])?;
        }
    }

    if branch_exists_remote(&opts.remote, branch_name) {
        run_git(&["push", &opts.remote, "--delete", branch_name])?;
    }

    Ok(())
}

fn run_finish_branch(opts: FinishBranchOptions) -> Result<(), String> {
    ensure_git_repo()?;

    let branch_name = match opts.branch_name {
        Some(name) => name,
        None => current_branch()?,
    };

    require_non_protected_branch(&branch_name)?;
    git_fetch_prune(&opts.remote)?;

    let current = current_branch()?;
    if current == branch_name {
        if branch_exists_local(&opts.base_branch) {
            run_git(&["checkout", &opts.base_branch])?;
        } else {
            let remote_ref = format!("{}/{}", opts.remote, opts.base_branch);
            run_git(&["checkout", "-b", &opts.base_branch, &remote_ref])?;
        }
        run_git(&["pull", &opts.remote, &opts.base_branch])?;
    }

    if branch_exists_local(&branch_name) && run_git(&["branch", "-d", &branch_name]).is_err() {
        run_git(&["branch", "-D", &branch_name])?;
    }

    if branch_exists_remote(&opts.remote, &branch_name) {
        let _ = run_git(&["push", &opts.remote, "--delete", &branch_name]);
    }

    git_fetch_prune(&opts.remote)?;
    Ok(())
}

fn run_create_after_delete(opts: CreateAfterDeleteOptions) -> Result<(), String> {
    ensure_git_repo()?;

    let branch_name = current_branch()?;
    require_non_protected_branch(&branch_name)?;

    git_fetch_prune(&opts.remote)?;
    run_git(&["checkout", &opts.base_branch])?;
    run_git(&["pull", &opts.remote, &opts.base_branch])?;

    if branch_exists_local(&branch_name) {
        run_git(&["branch", "-d", &branch_name])?;
    }

    if branch_exists_remote(&opts.remote, &branch_name) {
        run_git(&["push", &opts.remote, "--delete", &branch_name])?;
    }

    run_git(&["checkout", "-b", &branch_name, &opts.base_branch])?;
    run_git(&["push", "--set-upstream", &opts.remote, &branch_name])?;
    Ok(())
}

fn run_clean_local_gone(opts: CleanLocalGoneOptions) -> Result<(), String> {
    ensure_git_repo()?;
    git_fetch_prune(&opts.remote)?;

    for branch in list_gone_branches()? {
        if is_protected_branch(&branch) {
            continue;
        }
        if run_git(&["branch", "-d", &branch]).is_err() {
            let _ = run_git(&["branch", "-D", &branch]);
        }
    }

    Ok(())
}

fn run_clean_branches(opts: CleanBranchesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    git_fetch_prune(&opts.remote)?;

    for branch in list_gone_branches()? {
        if is_protected_branch(&branch) {
            continue;
        }

        if opts.dry_run {
            println!("[DRY-RUN] Would delete local branch: {branch}");
            continue;
        }

        if run_git(&["branch", "-d", &branch]).is_err() {
            let _ = run_git(&["branch", "-D", &branch]);
        }
    }

    let merged_output = run_git_output(&["branch", "--merged", &opts.base_branch])?;
    for line in merged_output.lines() {
        let branch = line.trim().trim_start_matches("* ").trim();
        if branch.is_empty() || is_protected_branch(branch) {
            continue;
        }
        println!("{branch}");
    }

    Ok(())
}

fn run_cleanup_after_pr(opts: CleanupAfterPrOptions) -> Result<(), String> {
    ensure_git_repo()?;

    let current_branch = current_branch().ok();

    run_git(&["checkout", &opts.base_branch])?;
    run_git(&["pull", &opts.remote, &opts.base_branch])?;
    git_fetch_prune(&opts.remote)?;

    let locals = run_git_output(&["for-each-ref", "--format=%(refname:short)", "refs/heads"])?;

    let mut outdated = Vec::new();
    for branch in locals.lines() {
        if branch.is_empty() || is_protected_branch(branch) {
            continue;
        }

        let range = format!("{branch}..{}", opts.base_branch);
        let behind_count = run_git_output(&["rev-list", "--count", &range])
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        if behind_count > 0 {
            outdated.push(branch.to_string());
        }
    }

    for branch in outdated {
        if run_git(&["branch", "-d", &branch]).is_err() {
            let _ = run_git(&["branch", "-D", &branch]);
        }

        if branch_exists_remote(&opts.remote, &branch) {
            let _ = run_git(&["push", &opts.remote, "--delete", &branch]);
        }

        if !opts.delete_only {
            run_git(&["checkout", "-b", &branch, &opts.base_branch])?;
            run_git(&["push", "--set-upstream", &opts.remote, &branch])?;
            run_git(&["checkout", &opts.base_branch])?;
        }
    }

    if let Some(branch) = current_branch
        && branch_exists_local(&branch)
    {
        let _ = run_git(&["checkout", &branch]);
    }

    Ok(())
}

fn ensure_git_repo() -> Result<(), String> {
    run_git(&["rev-parse", "--is-inside-work-tree"])
}

fn current_branch() -> Result<String, String> {
    let branch = run_git_output(&["branch", "--show-current"])?;
    if branch.is_empty() {
        return Err("Not on a branch (detached HEAD).".to_string());
    }
    Ok(branch)
}

fn run_git(args: &[&str]) -> Result<(), String> {
    git_cli::status(args).map_err(|err| format!("Failed to run git {}: {err}", args.join(" ")))
}

fn run_git_output(args: &[&str]) -> Result<String, String> {
    git_cli::output_trim(args).map_err(|err| format!("Failed to run git {}: {err}", args.join(" ")))
}

fn run_git_output_allow_failure(args: &[&str]) -> Option<String> {
    git_cli::output_trim(args).ok()
}

fn git_fetch_prune(remote: &str) -> Result<(), String> {
    run_git(&["fetch", remote, "--prune"])
}

fn branch_exists_local(branch_name: &str) -> bool {
    git_cli::status_success(&[
        "show-ref",
        "--verify",
        "--quiet",
        &format!("refs/heads/{branch_name}"),
    ])
}

fn branch_exists_remote(remote: &str, branch_name: &str) -> bool {
    git_cli::status_success(&["ls-remote", "--exit-code", "--heads", remote, branch_name])
}

fn is_protected_branch(branch_name: &str) -> bool {
    matches!(branch_name, "main" | "dev")
}

fn require_non_protected_branch(branch_name: &str) -> Result<(), String> {
    if is_protected_branch(branch_name) {
        return Err(format!("Cannot operate on protected branch: {branch_name}"));
    }
    Ok(())
}

fn validate_branch_name(branch_name: &str) -> Result<(), String> {
    if branch_name.trim().is_empty() {
        return Err("Branch name cannot be empty".to_string());
    }

    if branch_name.contains(' ') {
        return Err(format!(
            "Invalid branch name (contains spaces): '{branch_name}'"
        ));
    }

    let allowed_prefixes = [
        "feature/",
        "feat/",
        "fix/",
        "fixture/",
        "doc/",
        "docs/",
        "refactor/",
        "test/",
        "tests/",
        "chore/",
    ];

    if allowed_prefixes
        .iter()
        .any(|prefix| branch_name.starts_with(prefix))
    {
        Ok(())
    } else {
        Err(format!(
            "Invalid branch name '{branch_name}'. Must start with one of: {}",
            allowed_prefixes.join(", ")
        ))
    }
}

fn validate_branch_type(branch_type: &str) -> Result<(), String> {
    match branch_type {
        "feature" | "feat" | "fixture" | "fix" | "chore" | "refactor" | "doc" | "docs" | "test"
        | "tests" => Ok(()),
        _ => Err(format!(
            "Invalid type '{branch_type}'. Must be one of: feature, feat, fixture, fix, chore, refactor, doc, docs, test, tests"
        )),
    }
}

fn sanitize_description(description: &str) -> String {
    description
        .to_lowercase()
        .chars()
        .map(|ch| match ch {
            'a'..='z' | '0'..='9' | '-' => ch,
            ' ' | '_' => '-',
            _ => '\0',
        })
        .filter(|ch| *ch != '\0')
        .collect::<String>()
}

fn require_clean_tree() -> Result<(), String> {
    let unstaged_clean = git_cli::status_success(&["diff", "--quiet"]);
    let staged_clean = git_cli::status_success(&["diff", "--cached", "--quiet"]);

    if unstaged_clean && staged_clean {
        Ok(())
    } else {
        Err("Working tree is dirty. Commit/stash your changes first.".to_string())
    }
}

fn has_upstream() -> bool {
    git_cli::status_success(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
}

fn validate_commit_message(message: &str) -> Result<(), String> {
    let regex = match COMMIT_MESSAGE_FORMAT_REGEX.as_ref() {
        Ok(re) => re,
        Err(err) => return Err(format!("Invalid regex: {err}")),
    };
    if regex.is_match(message) {
        Ok(())
    } else {
        Err("Invalid commit message format. Expected '<type>(<scope>): <message>' or '<type>: <message>'".to_string())
    }
}

fn list_gone_branches() -> Result<Vec<String>, String> {
    let output = run_git_output(&["branch", "-vv"])?;
    let mut branches = Vec::new();

    for line in output.lines() {
        if !line.contains(": gone]") {
            continue;
        }

        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap_or_default();
        let branch = if first == "*" {
            parts.next().unwrap_or_default()
        } else {
            first
        };

        if !branch.is_empty() {
            branches.push(branch.to_string());
        }
    }

    Ok(branches)
}

fn last_deleted_branch_file() -> Option<PathBuf> {
    let git_dir = run_git_output_allow_failure(&["rev-parse", "--git-dir"])?;
    Some(PathBuf::from(git_dir).join("last_deleted_branch"))
}

fn save_last_deleted_branch(branch_name: &str) -> Result<(), String> {
    let Some(path) = last_deleted_branch_file() else {
        return Ok(());
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("Cannot create state dir: {err}"))?;
    }

    fs::write(path, branch_name).map_err(|err| format!("Cannot write state file: {err}"))
}

fn load_last_deleted_branch() -> Option<String> {
    let path = last_deleted_branch_file()?;
    let content = fs::read_to_string(path).ok()?;
    let value = content.trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}
