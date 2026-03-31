//! tools/versioning_automation/src/git/commands/git_action.rs
use crate::git::{
    commands::{
        AddCommitPushOptions, BranchCreationCheckOptions, CleanBranchesOptions,
        CleanLocalGoneOptions, CleanupAfterPrOptions, CreateAfterDeleteOptions,
        CreateBranchOptions, CreateWorkBranchOptions, DeleteBranchOptions, FinishBranchOptions,
        PushBranchOptions,
    },
    print_usage,
};

const DEFAULT_REMOTE: &str = "origin";
const DEFAULT_BASE_BRANCH: &str = "dev";

#[derive(Debug)]
pub(crate) enum GitAction {
    Help,
    CreateBranch(CreateBranchOptions),
    CreateWorkBranch(CreateWorkBranchOptions),
    PushBranch(PushBranchOptions),
    AddCommitPush(AddCommitPushOptions),
    DeleteBranch(DeleteBranchOptions),
    FinishBranch(FinishBranchOptions),
    CreateAfterDelete(CreateAfterDeleteOptions),
    CleanLocalGone(CleanLocalGoneOptions),
    CleanBranches(CleanBranchesOptions),
    CleanupAfterPr(CleanupAfterPrOptions),
    BranchCreationCheck(BranchCreationCheckOptions),
}

impl GitAction {
    pub(crate) fn run_action(self) -> i32 {
        let result = match self {
            Self::Help => {
                print_usage();
                Ok(())
            }
            Self::CreateBranch(opts) => CreateBranchOptions::run_create_branch(opts),
            Self::CreateWorkBranch(opts) => CreateWorkBranchOptions::run_create_work_branch(opts),
            Self::PushBranch(opts) => PushBranchOptions::run_push_branch(opts),
            Self::AddCommitPush(opts) => AddCommitPushOptions::run_add_commit_push(opts),
            Self::DeleteBranch(opts) => DeleteBranchOptions::run_delete_branch(opts),
            Self::FinishBranch(opts) => FinishBranchOptions::run_finish_branch(opts),
            Self::CreateAfterDelete(opts) => {
                CreateAfterDeleteOptions::run_create_after_delete(opts)
            }
            Self::CleanLocalGone(opts) => CleanLocalGoneOptions::run_clean_local_gone(opts),
            Self::CleanBranches(opts) => CleanBranchesOptions::run_clean_branches(opts),
            Self::CleanupAfterPr(opts) => CleanupAfterPrOptions::run_cleanup_after_pr(opts),
            Self::BranchCreationCheck(opts) => {
                BranchCreationCheckOptions::run_branch_creation_check(opts)
            }
        };

        match result {
            Ok(()) => 0,
            Err(message) => {
                eprintln!("{message}");
                1
            }
        }
    }

    pub(crate) fn parse(args: &[String]) -> Result<Self, String> {
        if args.is_empty() {
            return Ok(Self::Help);
        }

        match args[0].as_str() {
            "help" | "--help" | "-h" => Ok(Self::Help),
            "create-branch" => Self::parse_create_branch(&args[1..]),
            "create-work-branch" => Self::parse_create_work_branch(&args[1..]),
            "push-branch" => Self::parse_push_branch(&args[1..]),
            "add-commit-push" => Self::parse_add_commit_push(&args[1..]),
            "delete-branch" => Self::parse_delete_branch(&args[1..]),
            "finish-branch" => Self::parse_finish_branch(&args[1..]),
            "create-after-delete" => Self::parse_create_after_delete(&args[1..]),
            "clean-local-gone" => Self::parse_clean_local_gone(&args[1..]),
            "clean-branches" => Self::parse_clean_branches(&args[1..]),
            "cleanup-after-pr" => Self::parse_cleanup_after_pr(&args[1..]),
            "branch-creation-check" => Self::parse_branch_creation_check(&args[1..]),
            unknown => Err(format!("Unknown git subcommand: {unknown}")),
        }
    }

    pub(crate) fn run(args: &[String]) -> i32 {
        match Self::parse(args) {
            Ok(action) => Self::run_action(action),
            Err(message) => {
                eprintln!("{message}");
                2
            }
        }
    }

    fn parse_branch_creation_check(args: &[String]) -> Result<Self, String> {
        let command = args.first().cloned();
        let remaining = if args.len() > 1 {
            args[1..].to_vec()
        } else {
            Vec::new()
        };
        Ok(Self::BranchCreationCheck(BranchCreationCheckOptions {
            command,
            args: remaining,
        }))
    }

    fn parse_create_branch(args: &[String]) -> Result<Self, String> {
        let mut branch_name = None;
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
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
        Ok(Self::CreateBranch(CreateBranchOptions {
            branch_name,
            remote,
            base_branch,
        }))
    }

    fn parse_create_work_branch(args: &[String]) -> Result<Self, String> {
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
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }

        Ok(Self::CreateWorkBranch(CreateWorkBranchOptions {
            branch_type,
            description,
            remote,
            base_branch,
        }))
    }

    fn parse_push_branch(args: &[String]) -> Result<Self, String> {
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::PushBranch(PushBranchOptions { remote }))
    }

    fn parse_add_commit_push(args: &[String]) -> Result<Self, String> {
        if args.is_empty() {
            return Err(
                "Usage: add-commit-push <message> [--no-verify] [--remote <name>]".to_string(),
            );
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
                    remote = Self::required_value(args, i, "--remote")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::AddCommitPush(AddCommitPushOptions {
            message,
            no_verify,
            remote,
        }))
    }

    fn parse_delete_branch(args: &[String]) -> Result<Self, String> {
        if args.is_empty() {
            return Err(
                "Usage: delete-branch <name> [--force] [--remote <name>] [--base <branch>]"
                    .to_string(),
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
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::DeleteBranch(DeleteBranchOptions {
            branch_name,
            force,
            remote,
            base_branch,
        }))
    }

    fn parse_finish_branch(args: &[String]) -> Result<Self, String> {
        let mut branch_name = None;
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
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
        Ok(Self::FinishBranch(FinishBranchOptions {
            branch_name,
            remote,
            base_branch,
        }))
    }

    fn parse_create_after_delete(args: &[String]) -> Result<Self, String> {
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CreateAfterDelete(CreateAfterDeleteOptions {
            remote,
            base_branch,
        }))
    }

    fn parse_clean_local_gone(args: &[String]) -> Result<Self, String> {
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CleanLocalGone(CleanLocalGoneOptions { remote }))
    }

    fn parse_clean_branches(args: &[String]) -> Result<Self, String> {
        let mut dry_run = false;
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--dry-run" => dry_run = true,
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CleanBranches(CleanBranchesOptions {
            dry_run,
            remote,
            base_branch,
        }))
    }

    fn parse_cleanup_after_pr(args: &[String]) -> Result<Self, String> {
        let mut delete_only = false;
        let mut remote = DEFAULT_REMOTE.to_string();
        let mut base_branch = DEFAULT_BASE_BRANCH.to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--delete-only" => delete_only = true,
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CleanupAfterPr(CleanupAfterPrOptions {
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
}
