//! tools/versioning_automation/src/git/parse.rs
use crate::git::commands::{
    AddCommitPushOptions, CleanBranchesOptions, CleanLocalGoneOptions, CleanupAfterPrOptions,
    CreateAfterDeleteOptions, CreateBranchOptions, CreateWorkBranchOptions, DeleteBranchOptions,
    FinishBranchOptions, GitAction, PushBranchOptions,
};

const DEFAULT_REMOTE: &str = "origin";
const DEFAULT_BASE_BRANCH: &str = "dev";

pub(crate) fn parse(args: &[String]) -> Result<GitAction, String> {
    if args.is_empty() {
        return Ok(GitAction::Help);
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(GitAction::Help),
        "create-branch" => parse_create_branch(&args[1..]),
        "create-work-branch" => parse_create_work_branch(&args[1..]),
        "push-branch" => parse_push_branch(&args[1..]),
        "add-commit-push" => parse_add_commit_push(&args[1..]),
        "delete-branch" => parse_delete_branch(&args[1..]),
        "finish-branch" => parse_finish_branch(&args[1..]),
        "create-after-delete" => parse_create_after_delete(&args[1..]),
        "clean-local-gone" => parse_clean_local_gone(&args[1..]),
        "clean-branches" => parse_clean_branches(&args[1..]),
        "cleanup-after-pr" => parse_cleanup_after_pr(&args[1..]),
        unknown => Err(format!("Unknown git subcommand: {unknown}")),
    }
}

fn parse_create_branch(args: &[String]) -> Result<GitAction, String> {
    let mut branch_name = None;
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => {
                if branch_name.is_some() {
                    return Err(format!("Unexpected argument: {value}"));
                }
                branch_name = Some(value.to_string());
            }
        }
        i += 1;
    }
    Ok(GitAction::CreateBranch(CreateBranchOptions {
        branch_name,
        remote,
        base_branch,
    }))
}

fn parse_create_work_branch(args: &[String]) -> Result<GitAction, String> {
    if args.len() < 2 {
        return Err(
            "Usage: create-work-branch <type> <description> [--remote <name>] [--base <branch>]"
                .to_string(),
        );
    }
    let branch_type = args[0].clone();
    let description = args[1].clone();
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }

    Ok(GitAction::CreateWorkBranch(CreateWorkBranchOptions {
        branch_type,
        description,
        remote,
        base_branch,
    }))
}

fn parse_push_branch(args: &[String]) -> Result<GitAction, String> {
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::PushBranch(PushBranchOptions { remote }))
}

fn parse_add_commit_push(args: &[String]) -> Result<GitAction, String> {
    if args.is_empty() {
        return Err("Usage: add-commit-push <message> [--no-verify] [--remote <name>]".to_string());
    }
    let message = args[0].clone();
    let mut no_verify = false;
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--no-verify" => no_verify = true,
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::AddCommitPush(AddCommitPushOptions {
        message,
        no_verify,
        remote,
    }))
}

fn parse_delete_branch(args: &[String]) -> Result<GitAction, String> {
    if args.is_empty() {
        return Err(
            "Usage: delete-branch <name> [--force] [--remote <name>] [--base <branch>]".to_string(),
        );
    }
    let branch_name = args[0].clone();
    let mut force = false;
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--force" => force = true,
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::DeleteBranch(DeleteBranchOptions {
        branch_name,
        force,
        remote,
        base_branch,
    }))
}

fn parse_finish_branch(args: &[String]) -> Result<GitAction, String> {
    let mut branch_name = None;
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => {
                if branch_name.is_some() {
                    return Err(format!("Unexpected argument: {value}"));
                }
                branch_name = Some(value.to_string());
            }
        }
        i += 1;
    }
    Ok(GitAction::FinishBranch(FinishBranchOptions {
        branch_name,
        remote,
        base_branch,
    }))
}

fn parse_create_after_delete(args: &[String]) -> Result<GitAction, String> {
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::CreateAfterDelete(CreateAfterDeleteOptions {
        remote,
        base_branch,
    }))
}

fn parse_clean_local_gone(args: &[String]) -> Result<GitAction, String> {
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::CleanLocalGone(CleanLocalGoneOptions { remote }))
}

fn parse_clean_branches(args: &[String]) -> Result<GitAction, String> {
    let mut dry_run = false;
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" => dry_run = true,
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::CleanBranches(CleanBranchesOptions {
        dry_run,
        remote,
        base_branch,
    }))
}

fn parse_cleanup_after_pr(args: &[String]) -> Result<GitAction, String> {
    let mut delete_only = false;
    let mut remote = DEFAULT_REMOTE.to_string();
    let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--delete-only" => delete_only = true,
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(GitAction::CleanupAfterPr(CleanupAfterPrOptions {
        delete_only,
        remote,
        base_branch,
    }))
}

fn required_value(args: &[String], index: usize, option: &str) -> Result<String, String> {
    args.get(index)
        .cloned()
        .ok_or_else(|| format!("Option {option} requires a value."))
}
