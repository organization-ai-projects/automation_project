//! tools/versioning_automation/src/automation/commands/automation_action.rs
use std::env;

use crate::automation::commands::{
    AuditIssueStatusOptions, AuditSecurityOptions, BuildAccountsUiOptions,
    BuildAndCheckUiBundlesOptions, BuildUiBundlesOptions, ChangedCratesOptions,
    CheckDependenciesOptions, CheckMergeConflictsOptions, CheckPriorityIssuesOptions,
    CiWatchPrOptions, CleanArtifactsOptions, CommitMsgCheckOptions, InstallHooksOptions,
    LabelsSyncOptions, PostCheckoutCheckOptions, PreAddReviewOptions, PreBranchCreateCheckOptions,
    PreCommitCheckOptions, PrePushCheckOptions, PrepareCommitMsgOptions, ReleasePrepareOptions,
    SyncMainDevCiOptions, TestCoverageOptions,
};
use crate::automation::execute::{
    run_audit_security, run_post_checkout_check, run_test_coverage, to_exit_code,
};
use crate::automation::install_hooks::run_install_hooks;
use crate::automation::render::print_usage;
use crate::automation::{pre_add_review, run_check_dependencies, ui_build};

const DEFAULT_LABELS_FILE: &str = ".github/labels.json";

#[derive(Debug)]
pub(crate) enum AutomationAction {
    Help,
    AuditIssueStatus(AuditIssueStatusOptions),
    AuditSecurity(AuditSecurityOptions),
    BuildAccountsUi(BuildAccountsUiOptions),
    BuildUiBundles(BuildUiBundlesOptions),
    BuildAndCheckUiBundles(BuildAndCheckUiBundlesOptions),
    PreAddReview(PreAddReviewOptions),
    PreCommitCheck(PreCommitCheckOptions),
    PostCheckoutCheck(PostCheckoutCheckOptions),
    PrePushCheck(PrePushCheckOptions),
    ReleasePrepare(ReleasePrepareOptions),
    TestCoverage(TestCoverageOptions),
    ChangedCrates(ChangedCratesOptions),
    CheckMergeConflicts(CheckMergeConflictsOptions),
    CheckDependencies(CheckDependenciesOptions),
    CleanArtifacts(CleanArtifactsOptions),
    CommitMsgCheck(CommitMsgCheckOptions),
    InstallHooks(InstallHooksOptions),
    CheckPriorityIssues(CheckPriorityIssuesOptions),
    LabelsSync(LabelsSyncOptions),
    CiWatchPr(CiWatchPrOptions),
    SyncMainDevCi(SyncMainDevCiOptions),
    PrepareCommitMsg(PrepareCommitMsgOptions),
    PreBranchCreateCheck(PreBranchCreateCheckOptions),
}

impl AutomationAction {
    pub(crate) fn run_action(self) -> i32 {
        match self {
            Self::CommitMsgCheck(opts) => CommitMsgCheckOptions::run_commit_msg_check(opts),
            Self::Help => {
                print_usage();
                0
            }
            Self::AuditIssueStatus(opts) => {
                to_exit_code(AuditIssueStatusOptions::run_audit_issue_status(opts))
            }
            Self::AuditSecurity(_opts) => to_exit_code(run_audit_security()),
            Self::BuildAccountsUi(opts) => to_exit_code(ui_build::run_build_accounts_ui(opts)),
            Self::BuildUiBundles(opts) => to_exit_code(ui_build::run_build_ui_bundles(opts)),
            Self::BuildAndCheckUiBundles(opts) => {
                to_exit_code(ui_build::run_build_and_check_ui_bundles(opts))
            }
            Self::PreAddReview(_opts) => to_exit_code(pre_add_review::run_pre_add_review()),
            Self::PreCommitCheck(opts) => {
                to_exit_code(PreCommitCheckOptions::run_pre_commit_check(opts))
            }
            Self::PostCheckoutCheck(_opts) => to_exit_code(run_post_checkout_check()),
            Self::PrePushCheck(opts) => to_exit_code(PrePushCheckOptions::run_pre_push_check(opts)),
            Self::ReleasePrepare(opts) => {
                to_exit_code(ReleasePrepareOptions::run_release_prepare(opts))
            }
            Self::TestCoverage(_opts) => to_exit_code(run_test_coverage()),
            Self::ChangedCrates(opts) => {
                to_exit_code(ChangedCratesOptions::run_changed_crates(opts))
            }
            Self::CheckMergeConflicts(opts) => {
                to_exit_code(CheckMergeConflictsOptions::run_check_merge_conflicts(opts))
            }
            Self::CheckDependencies(opts) => to_exit_code(run_check_dependencies(opts)),
            Self::CleanArtifacts(opts) => {
                to_exit_code(CleanArtifactsOptions::run_clean_artifacts(opts))
            }
            Self::InstallHooks(_opts) => to_exit_code(run_install_hooks()),
            Self::CheckPriorityIssues(opts) => {
                to_exit_code(CheckPriorityIssuesOptions::run_check_priority_issues(opts))
            }
            Self::LabelsSync(opts) => to_exit_code(LabelsSyncOptions::run_labels_sync(opts)),
            Self::CiWatchPr(opts) => to_exit_code(CiWatchPrOptions::run_ci_watch_pr(opts)),
            Self::SyncMainDevCi(opts) => {
                to_exit_code(SyncMainDevCiOptions::run_sync_main_dev_ci(opts))
            }
            Self::PrepareCommitMsg(opts) => {
                to_exit_code(PrepareCommitMsgOptions::run_prepare_commit_msg(opts))
            }
            Self::PreBranchCreateCheck(opts) => to_exit_code(
                PreBranchCreateCheckOptions::run_pre_branch_create_check(opts),
            ),
        }
    }

    pub(crate) fn parse(args: &[String]) -> Result<Self, String> {
        if args.is_empty() {
            return Ok(Self::Help);
        }

        match args[0].as_str() {
            "help" | "--help" | "-h" => Ok(Self::Help),
            "audit-issue-status" => Self::parse_audit_issue_status(&args[1..]),
            "audit-security" => Self::parse_audit_security(&args[1..]),
            "build-accounts-ui" => Self::parse_build_accounts_ui(&args[1..]),
            "build-ui-bundles" => Self::parse_build_ui_bundles(&args[1..]),
            "build-and-check-ui-bundles" => Self::parse_build_and_check_ui_bundles(&args[1..]),
            "pre-add-review" => Self::parse_pre_add_review(&args[1..]),
            "pre-commit-check" => Self::parse_pre_commit_check(&args[1..]),
            "post-checkout-check" => Self::parse_post_checkout_check(&args[1..]),
            "pre-push-check" => Self::parse_pre_push_check(&args[1..]),
            "release-prepare" => Self::parse_release_prepare(&args[1..]),
            "test-coverage" => Self::parse_test_coverage(&args[1..]),
            "changed-crates" => Self::parse_changed_crates(&args[1..]),
            "check-merge-conflicts" => Self::parse_check_merge_conflicts(&args[1..]),
            "check-dependencies" => Self::parse_check_dependencies(&args[1..]),
            "clean-artifacts" => Self::parse_clean_artifacts(&args[1..]),
            "commit-msg-check" => Self::parse_commit_msg_check(&args[1..]),
            "install-hooks" => Self::parse_install_hooks(&args[1..]),
            "prepare-commit-msg" => Self::parse_prepare_commit_msg(&args[1..]),
            "pre-branch-create-check" => Self::parse_pre_branch_create_check(&args[1..]),
            "check-priority-issues" => Self::parse_check_priority_issues(&args[1..]),
            "labels-sync" => Self::parse_labels_sync(&args[1..]),
            "ci-watch-pr" => Self::parse_ci_watch_pr(&args[1..]),
            "sync-main-dev-ci" => Self::parse_sync_main_dev_ci(&args[1..]),
            unknown => Err(format!("Unknown automation subcommand: {unknown}")),
        }
    }

    fn parse_audit_issue_status(args: &[String]) -> Result<Self, String> {
        let mut repo = None;
        let mut base_ref = "origin/main".to_string();
        let mut head_ref = "origin/dev".to_string();
        let mut limit = 200usize;
        let mut output_file = None;
        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--repo" => {
                    i += 1;
                    repo = Some(Self::required_value(args, i, "--repo")?);
                }
                "--base" => {
                    i += 1;
                    base_ref = Self::required_value(args, i, "--base")?;
                }
                "--head" => {
                    i += 1;
                    head_ref = Self::required_value(args, i, "--head")?;
                }
                "--limit" => {
                    i += 1;
                    let raw = Self::required_value(args, i, "--limit")?;
                    limit = raw
                        .parse::<usize>()
                        .map_err(|_| "--limit requires a positive integer".to_string())?;
                    if limit == 0 {
                        return Err("--limit requires a positive integer".to_string());
                    }
                }
                "--output" => {
                    i += 1;
                    output_file = Some(Self::required_value(args, i, "--output")?);
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::AuditIssueStatus(AuditIssueStatusOptions {
            repo,
            base_ref,
            head_ref,
            limit,
            output_file,
        }))
    }

    fn parse_audit_security(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::AuditSecurity(AuditSecurityOptions))
    }

    fn parse_build_accounts_ui(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::BuildAccountsUi(BuildAccountsUiOptions))
    }

    fn parse_build_ui_bundles(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::BuildUiBundles(BuildUiBundlesOptions))
    }

    fn parse_build_and_check_ui_bundles(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::BuildAndCheckUiBundles(BuildAndCheckUiBundlesOptions))
    }

    fn parse_pre_add_review(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::PreAddReview(PreAddReviewOptions))
    }

    fn parse_pre_commit_check(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::PreCommitCheck(PreCommitCheckOptions))
    }

    fn parse_post_checkout_check(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::PostCheckoutCheck(PostCheckoutCheckOptions))
    }

    fn parse_pre_push_check(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::PrePushCheck(PrePushCheckOptions))
    }

    fn parse_test_coverage(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::TestCoverage(TestCoverageOptions))
    }

    fn parse_release_prepare(args: &[String]) -> Result<Self, String> {
        if args.is_empty() {
            return Err("release-prepare requires: <version> [--auto-changelog]".to_string());
        }
        let version = args[0].clone();
        let mut auto_changelog = false;
        let mut i = 1usize;
        while i < args.len() {
            match args[i].as_str() {
                "--auto-changelog" => auto_changelog = true,
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::ReleasePrepare(ReleasePrepareOptions {
            version,
            auto_changelog,
        }))
    }

    fn parse_changed_crates(args: &[String]) -> Result<Self, String> {
        let mut ref1: Option<String> = None;
        let mut ref2: Option<String> = None;
        let mut output_format = env::var("OUTPUT_FORMAT").ok();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--output-format" => {
                    i += 1;
                    output_format = Some(Self::required_value(args, i, "--output-format")?);
                }
                value if value.starts_with('-') => {
                    return Err(format!("Unexpected argument: {value}"));
                }
                value => {
                    if ref1.is_none() {
                        ref1 = Some(value.to_string());
                    } else if ref2.is_none() {
                        ref2 = Some(value.to_string());
                    } else {
                        return Err(format!("Unexpected argument: {value}"));
                    }
                }
            }
            i += 1;
        }
        Ok(Self::ChangedCrates(ChangedCratesOptions {
            ref1,
            ref2,
            output_format,
        }))
    }

    fn parse_check_merge_conflicts(args: &[String]) -> Result<Self, String> {
        let mut remote = env::var("REMOTE").unwrap_or_else(|_| "origin".to_string());
        let mut base_branch = env::var("BASE_BRANCH").unwrap_or_else(|_| "dev".to_string());
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--base-branch" | "--base" => {
                    i += 1;
                    base_branch = Self::required_value(args, i, "--base-branch")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CheckMergeConflicts(CheckMergeConflictsOptions {
            remote,
            base_branch,
        }))
    }

    fn parse_check_dependencies(args: &[String]) -> Result<Self, String> {
        let mut check_outdated = true;
        let mut check_unused = true;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--skip-outdated" => check_outdated = false,
                "--skip-unused" => check_unused = false,
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CheckDependencies(CheckDependenciesOptions {
            check_outdated,
            check_unused,
        }))
    }

    fn parse_clean_artifacts(args: &[String]) -> Result<Self, String> {
        let mut include_node_modules = true;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--skip-node-modules" => include_node_modules = false,
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CleanArtifacts(CleanArtifactsOptions {
            include_node_modules,
        }))
    }

    fn parse_install_hooks(args: &[String]) -> Result<Self, String> {
        if let Some(value) = args.first() {
            return Err(format!("Unexpected argument: {value}"));
        }
        Ok(Self::InstallHooks(InstallHooksOptions))
    }

    fn parse_commit_msg_check(args: &[String]) -> Result<Self, String> {
        let mut file = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--file" => {
                    i += 1;
                    file = Some(Self::required_value(args, i, "--file")?);
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        let file = file.ok_or_else(|| "commit-msg-check requires --file".to_string())?;
        Ok(Self::CommitMsgCheck(CommitMsgCheckOptions { file }))
    }

    fn parse_prepare_commit_msg(args: &[String]) -> Result<Self, String> {
        let mut file = None;
        let mut source = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--file" => {
                    i += 1;
                    file = Some(Self::required_value(args, i, "--file")?);
                }
                "--source" => {
                    i += 1;
                    source = Some(Self::required_value(args, i, "--source")?);
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        let file = file.ok_or_else(|| "prepare-commit-msg requires --file".to_string())?;
        Ok(Self::PrepareCommitMsg(PrepareCommitMsgOptions {
            file,
            source,
        }))
    }

    fn parse_pre_branch_create_check(args: &[String]) -> Result<Self, String> {
        let mut branch = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--branch" => {
                    i += 1;
                    branch = Some(Self::required_value(args, i, "--branch")?);
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        let branch =
            branch.ok_or_else(|| "pre-branch-create-check requires --branch".to_string())?;
        Ok(Self::PreBranchCreateCheck(PreBranchCreateCheckOptions {
            branch,
        }))
    }

    fn parse_check_priority_issues(args: &[String]) -> Result<Self, String> {
        let mut repo = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--repo" => {
                    i += 1;
                    repo = Some(Self::required_value(args, i, "--repo")?);
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::CheckPriorityIssues(CheckPriorityIssuesOptions {
            repo,
        }))
    }

    fn parse_labels_sync(args: &[String]) -> Result<Self, String> {
        let mut labels_file = DEFAULT_LABELS_FILE.to_string();
        let mut prune = false;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--prune" => prune = true,
                "--labels-file" => {
                    i += 1;
                    labels_file = Self::required_value(args, i, "--labels-file")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }
        Ok(Self::LabelsSync(LabelsSyncOptions { labels_file, prune }))
    }

    fn parse_ci_watch_pr(args: &[String]) -> Result<Self, String> {
        let mut pr_number = None;
        let mut poll_interval = env::var("POLL_INTERVAL")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(10);
        let mut max_wait = env::var("MAX_WAIT")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(3600);
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--pr" => {
                    i += 1;
                    pr_number = Some(Self::required_value(args, i, "--pr")?);
                }
                "--poll-interval" => {
                    i += 1;
                    poll_interval = Self::required_value(args, i, "--poll-interval")?
                        .parse::<u64>()
                        .map_err(|_| "--poll-interval must be a positive integer".to_string())?;
                }
                "--max-wait" => {
                    i += 1;
                    max_wait = Self::required_value(args, i, "--max-wait")?
                        .parse::<u64>()
                        .map_err(|_| "--max-wait must be a positive integer".to_string())?;
                }
                value => {
                    if pr_number.is_some() {
                        return Err(format!("Unexpected argument: {value}"));
                    }
                    pr_number = Some(value.to_string());
                }
            }
            i += 1;
        }
        Ok(Self::CiWatchPr(CiWatchPrOptions {
            pr_number,
            poll_interval,
            max_wait,
        }))
    }

    fn parse_sync_main_dev_ci(args: &[String]) -> Result<Self, String> {
        let mut remote = env::var("REMOTE").unwrap_or_else(|_| "origin".to_string());
        let mut main = env::var("MAIN").unwrap_or_else(|_| "main".to_string());
        let mut dev = env::var("DEV").unwrap_or_else(|_| "dev".to_string());
        let mut sync_branch = "sync/main-into-dev".to_string();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--remote" => {
                    i += 1;
                    remote = Self::required_value(args, i, "--remote")?;
                }
                "--main" => {
                    i += 1;
                    main = Self::required_value(args, i, "--main")?;
                }
                "--dev" => {
                    i += 1;
                    dev = Self::required_value(args, i, "--dev")?;
                }
                "--sync-branch" => {
                    i += 1;
                    sync_branch = Self::required_value(args, i, "--sync-branch")?;
                }
                value => return Err(format!("Unexpected argument: {value}")),
            }
            i += 1;
        }

        Ok(Self::SyncMainDevCi(SyncMainDevCiOptions {
            remote,
            main,
            dev,
            sync_branch,
        }))
    }

    fn required_value(args: &[String], index: usize, option: &str) -> Result<String, String> {
        args.get(index)
            .cloned()
            .ok_or_else(|| format!("Option {option} requires a value."))
    }
}
