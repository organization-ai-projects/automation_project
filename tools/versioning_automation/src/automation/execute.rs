//! tools/versioning_automation/src/automation/execute.rs
use std::collections::{self, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use common_json::Json;
use regex::Regex;

use crate::automation::commands::{
    AuditSecurityOptions, AutomationAction, CiWatchPrOptions, CommitMsgCheckOptions,
    PostCheckoutCheckOptions, PreBranchCreateCheckOptions, PrepareCommitMsgOptions,
    SyncMainDevCiOptions, TestCoverageOptions,
};
use crate::automation::parse::parse;
use crate::automation::render::print_usage;
use crate::automation::{
    audit_issue_status, changed_crates, check_dependencies, check_merge_conflicts, hook_checks,
    install_hooks, pre_add_review, ui_build,
};
use crate::pr::text_payload::extract_effective_issue_ref_records;
use crate::repo_name::resolve_repo_name_optional;
use crate::{gh_cli, git_cli};

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(action) => run_action(action),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_action(action: AutomationAction) -> i32 {
    match action {
        AutomationAction::CommitMsgCheck(opts) => run_commit_msg_check(opts),
        AutomationAction::Help => {
            print_usage();
            0
        }
        AutomationAction::AuditIssueStatus(opts) => {
            to_exit_code(audit_issue_status::run_audit_issue_status(opts))
        }
        AutomationAction::AuditSecurity(opts) => to_exit_code(run_audit_security(opts)),
        AutomationAction::BuildAccountsUi(opts) => {
            to_exit_code(ui_build::run_build_accounts_ui(opts))
        }
        AutomationAction::BuildUiBundles(opts) => {
            to_exit_code(ui_build::run_build_ui_bundles(opts))
        }
        AutomationAction::BuildAndCheckUiBundles(opts) => {
            to_exit_code(ui_build::run_build_and_check_ui_bundles(opts))
        }
        AutomationAction::PreAddReview(opts) => {
            to_exit_code(pre_add_review::run_pre_add_review(opts))
        }
        AutomationAction::PreCommitCheck(opts) => {
            to_exit_code(hook_checks::run_pre_commit_check(opts))
        }
        AutomationAction::PostCheckoutCheck(opts) => to_exit_code(run_post_checkout_check(opts)),
        AutomationAction::PrePushCheck(opts) => to_exit_code(hook_checks::run_pre_push_check(opts)),
        AutomationAction::ReleasePrepare(opts) => {
            to_exit_code(super::release_prepare::run_release_prepare(opts))
        }
        AutomationAction::TestCoverage(opts) => to_exit_code(run_test_coverage(opts)),
        AutomationAction::ChangedCrates(opts) => {
            to_exit_code(changed_crates::run_changed_crates(opts))
        }
        AutomationAction::CheckMergeConflicts(opts) => {
            to_exit_code(check_merge_conflicts::run_check_merge_conflicts(opts))
        }
        AutomationAction::CheckDependencies(opts) => {
            to_exit_code(check_dependencies::run_check_dependencies(opts))
        }
        AutomationAction::CleanArtifacts(opts) => {
            to_exit_code(super::clean_artifacts::run_clean_artifacts(opts))
        }
        AutomationAction::InstallHooks(opts) => {
            to_exit_code(install_hooks::run_install_hooks(opts))
        }
        AutomationAction::CheckPriorityIssues(opts) => to_exit_code(
            super::check_priority_issues::run_check_priority_issues(opts),
        ),
        AutomationAction::LabelsSync(opts) => {
            to_exit_code(super::labels_sync::run_labels_sync(opts))
        }
        AutomationAction::CiWatchPr(opts) => to_exit_code(run_ci_watch_pr(opts)),
        AutomationAction::SyncMainDevCi(opts) => to_exit_code(run_sync_main_dev_ci(opts)),
        AutomationAction::PrepareCommitMsg(opts) => to_exit_code(run_prepare_commit_msg(opts)),
        AutomationAction::PreBranchCreateCheck(opts) => {
            to_exit_code(run_pre_branch_create_check(opts))
        }
    }
}

fn to_exit_code(result: Result<(), String>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn run_commit_msg_check(opts: CommitMsgCheckOptions) -> i32 {
    const RC_INVALID_FORMAT: i32 = 3;
    const RC_MIXED_CATEGORY: i32 = 6;
    const RC_SCOPE_MISSING: i32 = 7;
    const RC_SCOPE_MISMATCH: i32 = 8;

    if std::env::var("SKIP_COMMIT_VALIDATION").unwrap_or_default() == "1" {
        return 0;
    }

    let commit_msg_path = PathBuf::from(&opts.file);
    if !commit_msg_path.is_file() {
        eprintln!("commit-msg-check: missing or invalid --file");
        return RC_INVALID_FORMAT;
    }

    let message = fs::read_to_string(&commit_msg_path)
        .map_err(|e| format!("Failed to read '{}': {e}", commit_msg_path.display()));
    let Ok(message) = message else {
        eprintln!("{}", message.unwrap_err());
        return RC_INVALID_FORMAT;
    };
    let subject = first_non_comment_subject_line(&message);
    let subject = subject.as_deref().unwrap_or_default();

    match parse_subject_max_len() {
        Ok(Some(max_len)) if subject.chars().count() > max_len => {
            eprintln!(
                "Commit subject too long: {}/{} characters.",
                subject.chars().count(),
                max_len
            );
            return 9;
        }
        Ok(_) => {}
        Err(message) => {
            eprintln!("{message}");
            return RC_INVALID_FORMAT;
        }
    }

    let format_re =
        Regex::new(r"^(feature|feat|fix|doc|docs|refactor|test|tests|chore|perf)(\([a-zA-Z0-9_./,-]+\))?:[[:space:]].+$")
            .expect("static regex must compile");
    if !format_re.is_match(subject) {
        eprintln!("Invalid commit message format: '{subject}'");
        return RC_INVALID_FORMAT;
    }

    let footer_status = crate::issues::run(&[
        "validate-footer".to_string(),
        "--file".to_string(),
        opts.file.clone(),
    ]);
    if footer_status != 0 {
        return footer_status;
    }

    let staged_files_text =
        match run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRUD"])
        {
            Ok(value) => value,
            Err(message) => {
                eprintln!("{message}");
                return 1;
            }
        };
    let staged_files = staged_files_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let format_categories = collect_format_categories(&staged_files);
    if format_categories.len() > 1 {
        eprintln!(
            "Mixed file format categories are not allowed in one commit: {}",
            format_categories.join(", ")
        );
        return RC_MIXED_CATEGORY;
    }

    let required_scopes = match detect_required_scopes(&staged_files) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{message}");
            return 1;
        }
    };
    if !required_scopes.is_empty() {
        let commit_scopes = extract_scopes_from_commit_subject(subject);
        if commit_scopes.is_empty() {
            eprintln!("Missing required scope in commit message.");
            return RC_SCOPE_MISSING;
        }
        let missing = required_scopes
            .into_iter()
            .filter(|required| !commit_scopes.iter().any(|scope| scope == required))
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            eprintln!(
                "Commit scope does not match touched files. Missing: {}",
                missing.join(", ")
            );
            return RC_SCOPE_MISMATCH;
        }
    }

    0
}

fn run_prepare_commit_msg(opts: PrepareCommitMsgOptions) -> Result<(), String> {
    if std::env::var("SKIP_PREPARE_COMMIT_MSG").unwrap_or_default() == "1" {
        return Ok(());
    }

    let path = PathBuf::from(&opts.file);
    if !path.is_file() {
        return Ok(());
    }
    let source = opts.source.unwrap_or_default();
    if matches!(source.as_str(), "message" | "merge" | "squash" | "commit") {
        return Ok(());
    }

    let current = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
    if has_non_comment_content(&current) {
        return Ok(());
    }

    let branch = run_git_output(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default();
    if branch.trim().is_empty() || branch.trim() == "HEAD" {
        return Ok(());
    }

    let staged_files_text =
        run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRU"])?;
    let staged_files = staged_files_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if staged_files.is_empty() {
        return Ok(());
    }

    let (commit_type, fallback_warning) = detect_commit_type_from_context(&staged_files);
    let scopes = detect_required_scopes(&staged_files)?;
    let scopes_csv = scopes.join(",");
    let description = derive_description(branch.trim(), &staged_files);
    let subject = if scopes_csv.is_empty() {
        format!("{commit_type}(workspace): {description}")
    } else {
        format!("{commit_type}({scopes_csv}): {description}")
    };

    let mut rendered = String::new();
    rendered.push_str(&subject);
    rendered.push_str("\n\n");
    rendered.push_str("# Auto-generated from branch and staged files.\n");
    if let Some(warning) = fallback_warning {
        rendered.push_str(warning);
        rendered.push('\n');
    }
    rendered.push_str("# Edit freely before saving this commit.\n");

    fs::write(&path, rendered).map_err(|e| format!("Failed to write '{}': {e}", path.display()))?;
    Ok(())
}

fn run_pre_branch_create_check(opts: PreBranchCreateCheckOptions) -> Result<(), String> {
    let branch = opts.branch.trim();
    if branch.is_empty() {
        return Err("No branch name provided.".to_string());
    }
    let re = Regex::new(r"^(feature|fix|hotfix|release)/[a-zA-Z0-9_-]+$")
        .map_err(|e| format!("Invalid branch regex: {e}"))?;
    if !re.is_match(branch) {
        return Err(format!(
            "Invalid branch name '{branch}'. Expected (feature|fix|hotfix|release)/<name>"
        ));
    }
    if branch_exists_local(branch) {
        return Err(format!("Branch '{branch}' already exists locally."));
    }
    let marker = format!("[{branch}]");
    let worktrees = run_git_output(&["worktree", "list"])?;
    if worktrees.lines().any(|line| line.contains(&marker)) {
        return Err(format!(
            "Branch '{branch}' is already in use by another worktree."
        ));
    }
    println!("Branch name '{branch}' is valid.");
    Ok(())
}

fn first_non_comment_subject_line(message: &str) -> Option<String> {
    message
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToString::to_string)
}

fn parse_subject_max_len() -> Result<Option<usize>, String> {
    let raw = std::env::var("COMMIT_MSG_SUBJECT_MAX_LEN").unwrap_or_default();
    if raw.trim().is_empty() {
        return Ok(None);
    }
    let parsed = raw
        .trim()
        .parse::<usize>()
        .map_err(|_| "COMMIT_MSG_SUBJECT_MAX_LEN must be a positive integer".to_string())?;
    if parsed == 0 {
        return Ok(None);
    }
    Ok(Some(parsed))
}

fn has_non_comment_content(message: &str) -> bool {
    message
        .lines()
        .map(str::trim)
        .any(|line| !line.is_empty() && !line.starts_with('#'))
}

fn collect_format_categories(staged_files: &[String]) -> Vec<String> {
    let mut categories = BTreeSet::new();
    for file in staged_files {
        if is_shell_file_path(file) {
            categories.insert("shell".to_string());
            continue;
        }
        if file.ends_with(".md") {
            categories.insert("markdown".to_string());
            continue;
        }
        if file.ends_with(".rs") || file.ends_with("/Cargo.toml") || file == "Cargo.toml" {
            categories.insert("rust".to_string());
            continue;
        }
        categories.insert("other".to_string());
    }
    categories.into_iter().collect()
}

fn detect_required_scopes(staged_files: &[String]) -> Result<Vec<String>, String> {
    let root = repo_root()?;
    let mut scopes = BTreeSet::new();
    for file in staged_files {
        if let Some(scope) = resolve_scope_from_file_path(&root, file) {
            scopes.insert(scope);
        }
    }
    if !scopes.is_empty() {
        return Ok(scopes.into_iter().collect());
    }

    if !staged_files.is_empty() && staged_files.iter().all(|f| is_shell_file_path(f)) {
        return Ok(vec!["shell".to_string()]);
    }
    if !staged_files.is_empty() && staged_files.iter().all(|f| f.ends_with(".md")) {
        return Ok(vec!["markdown".to_string()]);
    }
    if staged_files.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![common_path_scope(staged_files)])
}

pub(crate) fn resolve_scope_from_file_path(root: &Path, file: &str) -> Option<String> {
    if let Some(crate_dir) = find_crate_dir_for_file(root, file) {
        return Some(crate_dir);
    }
    if let Some(scope) = resolve_library_scope(file) {
        return Some(scope);
    }
    if let Some(scope) = resolve_product_scope(root, file) {
        return Some(scope);
    }
    if let Some(scope) = resolve_tools_scope(file) {
        return Some(scope);
    }
    None
}

fn resolve_library_scope(file: &str) -> Option<String> {
    let parts = file.split('/').collect::<Vec<_>>();
    if parts.len() < 4 || parts[0] != "projects" || parts[1] != "libraries" {
        return None;
    }
    Some(format!("{}/{}/{}", parts[0], parts[1], parts[2]))
}

fn resolve_product_scope(root: &Path, file: &str) -> Option<String> {
    let parts = file.split('/').collect::<Vec<_>>();
    if parts.len() < 5
        || parts[0] != "projects"
        || parts[1] != "products"
        || (parts[2] != "stable" && parts[2] != "unstable")
    {
        return None;
    }
    let base = format!("{}/{}/{}/{}", parts[0], parts[1], parts[2], parts[3]);
    if let Some(crate_dir) = find_crate_dir_for_file(root, file) {
        let base_prefix = format!("{base}/");
        if let Some(rest) = crate_dir.strip_prefix(&base_prefix) {
            let component = rest.split('/').next().unwrap_or_default();
            if !component.is_empty() {
                return Some(format!("{base}/{component}"));
            }
        }
    }
    Some(base)
}

fn resolve_tools_scope(file: &str) -> Option<String> {
    let parts = file.split('/').collect::<Vec<_>>();
    if parts.len() < 2 || parts[0] != "tools" {
        return None;
    }
    Some(format!("{}/{}", parts[0], parts[1]))
}

fn common_path_scope(staged_files: &[String]) -> String {
    let mut common = String::new();
    for file in staged_files {
        let dir = match file.rfind('/') {
            Some(index) => &file[..index],
            None => ".",
        };
        if common.is_empty() {
            common = dir.to_string();
            continue;
        }
        while common != "."
            && common != dir
            && !dir.starts_with(&(common.clone() + "/"))
            && !common.is_empty()
        {
            if let Some(index) = common.rfind('/') {
                common.truncate(index);
            } else {
                common = ".".to_string();
                break;
            }
        }
    }
    if common.is_empty() || common == "." {
        "workspace".to_string()
    } else {
        common
    }
}

fn extract_scopes_from_commit_subject(subject: &str) -> Vec<String> {
    let re = Regex::new(r"^[a-z]+\(([^)]+)\):").expect("scope extraction regex must compile");
    let Some(captures) = re.captures(subject) else {
        return Vec::new();
    };
    let Some(raw) = captures.get(1) else {
        return Vec::new();
    };
    raw.as_str()
        .split(',')
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn detect_commit_type_from_context(staged_files: &[String]) -> (String, Option<&'static str>) {
    if is_docs_only_change(staged_files) {
        return ("docs".to_string(), None);
    }
    if is_tests_only_change(staged_files) {
        return ("test".to_string(), None);
    }
    (
        "type".to_string(),
        Some(
            "# WARNING: type not inferred from staged files; replace 'type' with feat/fix/refactor/chore/etc.",
        ),
    )
}

fn is_docs_only_change(staged_files: &[String]) -> bool {
    !staged_files.is_empty()
        && staged_files.iter().all(|file| {
            file.ends_with(".md") || file.starts_with("documentation/") || file.starts_with("docs/")
        })
}

fn is_tests_only_change(staged_files: &[String]) -> bool {
    !staged_files.is_empty()
        && staged_files.iter().all(|file| {
            file.contains("/tests/")
                || file.starts_with("tests/")
                || file.ends_with("_test.rs")
                || file.ends_with(".snap")
        })
}

fn derive_description(branch: &str, staged_files: &[String]) -> String {
    let mut name = branch.to_string();
    for prefix in [
        "feat/",
        "feature/",
        "fix/",
        "hotfix/",
        "bugfix/",
        "docs/",
        "doc/",
        "refactor/",
        "test/",
        "tests/",
        "chore/",
        "perf/",
    ] {
        if let Some(rest) = name.strip_prefix(prefix) {
            name = rest.to_string();
            break;
        }
    }
    if let Ok(re) = Regex::new(r"^[A-Za-z]+-[0-9]+[-_/]")
        && let Some(m) = re.find(&name)
    {
        name = name[m.end()..].to_string();
    }
    let normalized = name.replace(['/', '_', '-'], " ");
    let normalized = normalized
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    if !normalized.is_empty() {
        return normalized;
    }
    if let Some(first_file) = staged_files.first() {
        let stem = Path::new(first_file)
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("changes")
            .replace(['_', '-'], " ");
        let stem = stem.split_whitespace().collect::<Vec<_>>().join(" ");
        return format!("update {stem}");
    }
    "update changes".to_string()
}

fn run_audit_security(_opts: AuditSecurityOptions) -> Result<(), String> {
    ensure_git_repo()?;
    if !command_available("cargo-audit") {
        return Err("cargo-audit not found. Install with: cargo install cargo-audit".to_string());
    }
    let _ = run_command_status("cargo", &["audit", "fetch"], true);
    run_command_status("cargo", &["audit"], false)
}

fn run_post_checkout_check(_opts: PostCheckoutCheckOptions) -> Result<(), String> {
    let upstream_branch = resolve_upstream_or_default();

    let commit_messages =
        run_git_output_preserve(&["log", &format!("{upstream_branch}..HEAD"), "--format=%B"])
            .unwrap_or_default();
    if commit_messages.trim().is_empty() {
        return Ok(());
    }

    let refs = extract_issue_refs_hook_detailed(&commit_messages)?;
    if refs.is_empty() {
        return Ok(());
    }

    let Some(repo_name) = resolve_repo_name_optional(None) else {
        return Ok(());
    };

    let mut root_parent_refs: Vec<String> = Vec::new();
    for (action, issue_number) in refs {
        if issue_is_root_parent(&issue_number, &repo_name)? {
            root_parent_refs.push(format!("{action} #{issue_number}"));
        }
    }

    if !root_parent_refs.is_empty() {
        println!();
        println!("⚠️  Convention warning on branch checkout:");
        println!("   Current branch history references root parent issue(s):");
        for parent_ref in &root_parent_refs {
            println!("   - {parent_ref}");
        }
        println!("   This will be blocked by commit-msg/pre-push checks for new commits.");
        println!("   Use child issue refs in trailers instead.");
        println!();
    }

    Ok(())
}

fn run_test_coverage(_opts: TestCoverageOptions) -> Result<(), String> {
    ensure_git_repo()?;
    require_command(
        "cargo-tarpaulin",
        "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin",
    )?;

    let coverage_dir = repo_root()?.join("target").join("coverage");
    fs::create_dir_all(&coverage_dir).map_err(|e| {
        format!(
            "Failed to create coverage output directory '{}': {e}",
            coverage_dir.display()
        )
    })?;

    let formats_raw = std::env::var("COVERAGE_FORMATS").unwrap_or_else(|_| "html".to_string());
    let include_lcov = formats_raw.to_lowercase().contains("lcov");
    let include_json = formats_raw.to_lowercase().contains("json");

    let mut args = vec![
        "tarpaulin",
        "--workspace",
        "--all-features",
        "--out",
        "Html",
        "--output-dir",
        "target/coverage",
        "--timeout",
        "300",
        "--exclude-files",
        "*/tests/*",
        "--exclude-files",
        "*/benches/*",
    ];
    if include_lcov {
        args.extend(["--out", "Lcov"]);
    }
    if include_json {
        args.extend(["--out", "Json"]);
    }
    run_command_status("cargo", &args, false)?;
    println!(
        "Coverage report generated: {}/index.html",
        coverage_dir.display()
    );
    Ok(())
}

fn run_ci_watch_pr(opts: CiWatchPrOptions) -> Result<(), String> {
    let pr_number = match opts.pr_number {
        Some(value) => value,
        None => {
            let branch = run_git_output(&["branch", "--show-current"])?;
            if branch.trim().is_empty() {
                return Err("No PR provided and unable to detect current branch.".to_string());
            }
            let value = run_gh_output(&[
                "pr",
                "list",
                "--head",
                branch.trim(),
                "--json",
                "number",
                "--jq",
                ".[0].number",
            ])?;
            if value.trim().is_empty() || value.trim() == "null" {
                return Err(format!("No PR found for branch '{}'.", branch.trim()));
            }
            value.trim().to_string()
        }
    };

    let start = Instant::now();
    loop {
        if start.elapsed().as_secs() > opts.max_wait {
            return Err(format!("Timeout: CI not complete after {}s", opts.max_wait));
        }

        let output = run_gh_output(&[
            "pr",
            "view",
            &pr_number,
            "--json",
            "state,mergeable,statusCheckRollup",
        ])?;
        let parsed = parse_json_object(&output, "PR JSON")?;
        let checks = parsed
            .get("statusCheckRollup")
            .and_then(Json::as_array)
            .cloned()
            .unwrap_or_default();
        let total = checks.len();

        if total == 0 {
            thread::sleep(Duration::from_secs(opts.poll_interval));
            continue;
        }

        let pending = checks
            .iter()
            .filter(|check| {
                check
                    .as_object()
                    .and_then(|object| object.get("conclusion"))
                    .and_then(Json::as_str)
                    .is_none()
            })
            .count();
        let success = checks
            .iter()
            .filter(|check| {
                check
                    .as_object()
                    .and_then(|object| object.get("conclusion"))
                    .and_then(Json::as_str)
                    == Some("SUCCESS")
            })
            .count();
        let failure = checks
            .iter()
            .filter(|check| {
                check
                    .as_object()
                    .and_then(|object| object.get("conclusion"))
                    .and_then(Json::as_str)
                    == Some("FAILURE")
            })
            .count();
        let state = object_string_or_default(&parsed, "state", "UNKNOWN");
        let mergeable = object_string_or_default(&parsed, "mergeable", "UNKNOWN");

        println!(
            "[{} s] State: {} | Mergeable: {} | Checks: {}/{} passed, {} failed, {} pending",
            start.elapsed().as_secs(),
            state,
            mergeable,
            success,
            total,
            failure,
            pending
        );

        if failure > 0 {
            return Err(format!("CI failed for PR #{}.", pr_number));
        }

        if pending == 0 && success == total {
            println!("All checks passed for PR #{}.", pr_number);
            break;
        }

        thread::sleep(Duration::from_secs(opts.poll_interval));
    }

    Ok(())
}

fn run_sync_main_dev_ci(opts: SyncMainDevCiOptions) -> Result<(), String> {
    if std::env::var("CI").unwrap_or_default() != "true" {
        return Err("This command can only be executed in CI (CI=true).".to_string());
    }

    run_git(&["fetch", &opts.remote])?;

    let main_ref = format!("{}/{}", opts.remote, opts.main);
    let dev_ref = format!("{}/{}", opts.remote, opts.dev);

    let main_sha = run_git_output(&["rev-parse", &main_ref])?;
    let dev_sha = run_git_output(&["rev-parse", &dev_ref])?;
    if main_sha == dev_sha {
        println!("No sync needed - dev is already up to date with main");
        return Ok(());
    }

    if run_git(&["merge-base", "--is-ancestor", &main_ref, &dev_ref]).is_ok() {
        println!("No sync needed - dev already contains all commits from main");
        return Ok(());
    }

    if branch_exists_local(&opts.sync_branch) {
        let _ = run_git(&["branch", "-D", &opts.sync_branch]);
    }
    if branch_exists_remote(&opts.remote, &opts.sync_branch) {
        let _ = run_git(&["push", &opts.remote, "--delete", &opts.sync_branch]);
    }

    run_git(&["switch", "-C", &opts.sync_branch, &main_ref])?;
    run_git(&["push", "-f", &opts.remote, &opts.sync_branch])?;

    let pr_output = run_gh_output(&[
        "pr",
        "create",
        "--base",
        &opts.dev,
        "--head",
        &opts.sync_branch,
        "--title",
        "chore: sync main into dev",
        "--body",
        "Automated sync after merge into main.",
    ])?;

    let pr_url = pr_output.trim().to_string();
    if pr_url.is_empty() {
        return Err("Failed to create sync PR (empty response).".to_string());
    }

    let stable_timeout = std::env::var("STABLE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(120);
    let deadline = Instant::now() + Duration::from_secs(stable_timeout);

    let mergeable = loop {
        if Instant::now() >= deadline {
            return Err("PR did not stabilize in time.".to_string());
        }
        let value = run_gh_output(&[
            "pr",
            "view",
            &pr_url,
            "--json",
            "mergeable",
            "--jq",
            ".mergeable // \"UNKNOWN\"",
        ])?;
        if value != "UNKNOWN" {
            break value;
        }
        thread::sleep(Duration::from_secs(5));
    };

    if mergeable == "CONFLICTING" {
        return Err("PR has merge conflicts. Cannot enable auto-merge.".to_string());
    }
    if mergeable != "MERGEABLE" {
        return Err(format!("PR is not mergeable (status: {mergeable})."));
    }

    run_gh_status(&[
        "pr",
        "merge",
        &pr_url,
        "--auto",
        "--merge",
        "--delete-branch",
    ])?;
    Ok(())
}

pub(crate) fn validate_no_root_parent_refs(
    commits: &str,
    repo: Option<&str>,
) -> Result<(), String> {
    let Some(repo_name) = repo else {
        if remote_policy_warn_only() {
            println!("Remote footer check skipped (repo unresolved, warn-only mode).");
            return Ok(());
        }
        return Err("Cannot resolve GitHub repository for footer validation.".to_string());
    };
    let refs = extract_issue_refs_detailed(commits)?;
    for (_action, issue) in refs {
        if issue_is_root_parent(&issue, repo_name)? {
            return Err(format!(
                "Root parent issue reference detected in commits: #{}",
                issue
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_part_of_only_policy(
    commits: &str,
    repo: Option<&str>,
) -> Result<(), String> {
    let refs = extract_issue_refs_detailed(commits)?;
    if refs.is_empty() {
        return Ok(());
    }
    let Some(repo_name) = repo else {
        if remote_policy_warn_only()
            || std::env::var("ALLOW_PART_OF_ONLY_PUSH").unwrap_or_default() == "1"
        {
            println!("Assignment policy check skipped (repo unresolved).");
            return Ok(());
        }
        return Err("Cannot resolve GitHub repository for Part-of assignment policy.".to_string());
    };
    let current_login = run_gh_output(&["api", "user", "--jq", ".login"]).unwrap_or_default();
    if current_login.trim().is_empty() {
        if remote_policy_warn_only()
            || std::env::var("ALLOW_PART_OF_ONLY_PUSH").unwrap_or_default() == "1"
        {
            println!("Assignment policy check skipped (login unresolved).");
            return Ok(());
        }
        return Err("Cannot resolve current GitHub login for assignment policy.".to_string());
    }

    let mut has_part_of = BTreeSet::new();
    let mut has_closing = BTreeSet::new();
    for (action, issue) in refs {
        if action == "part of" {
            has_part_of.insert(issue.clone());
        }
        if action == "closes" || action == "fixes" {
            has_closing.insert(issue);
        }
    }
    let mut violations = Vec::new();
    for issue in has_part_of {
        if has_closing.contains(&issue) {
            continue;
        }
        let assignees = run_gh_output(&[
            "issue",
            "view",
            &issue,
            "-R",
            repo_name,
            "--json",
            "assignees",
            "--jq",
            ".assignees[].login",
        ])
        .unwrap_or_default();
        let logins = assignees
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if logins.len() == 1 && logins[0] == current_login.trim() {
            violations.push(issue);
        }
    }
    if !violations.is_empty() && std::env::var("ALLOW_PART_OF_ONLY_PUSH").unwrap_or_default() != "1"
    {
        return Err(format!(
            "Push blocked by assignment policy for issues: {}",
            violations
                .iter()
                .map(|v| format!("#{v}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    Ok(())
}

fn remote_policy_warn_only() -> bool {
    let hooks_policy = std::env::var("HOOKS_REMOTE_POLICY").unwrap_or_default();
    let allow_offline = std::env::var("ALLOW_OFFLINE_REMOTE_CHECKS").unwrap_or_default();
    hooks_policy == "warn" || allow_offline == "1"
}

fn extract_issue_refs_detailed(text: &str) -> Result<Vec<(String, String)>, String> {
    extract_issue_refs_hook_detailed(text)
}

fn extract_issue_refs_hook_detailed(text: &str) -> Result<Vec<(String, String)>, String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for record in extract_effective_issue_ref_records(text) {
        let action = match record.first.as_str() {
            "Part of" => "part of".to_string(),
            "Closes" => "closes".to_string(),
            "Reopen" => "reopen".to_string(),
            _ => continue,
        };
        let issue = record.second.trim_start_matches('#').to_string();
        if issue.is_empty() {
            continue;
        }
        let key = format!("{action}|{issue}");
        if seen.insert(key) {
            out.push((action, issue));
        }
    }
    Ok(out)
}

fn issue_is_root_parent(issue_number: &str, repo: &str) -> Result<bool, String> {
    let body = run_gh_output(&[
        "issue",
        "view",
        issue_number,
        "-R",
        repo,
        "--json",
        "body",
        "--jq",
        ".body // \"\"",
    ])?;
    let parent = super::audit_issue_status::extract_parent_field(&body)
        .unwrap_or_else(|| "none".to_string())
        .to_lowercase();
    if parent == "epic" {
        return Ok(true);
    }
    if parent == "base" || parent.starts_with('#') {
        return Ok(false);
    }

    let (owner, repo_name) = split_repo_owner_name(repo)?;
    let has_children =
        !extract_subissue_refs_for_parent(&owner, &repo_name, issue_number)?.is_empty();
    Ok(has_children)
}

fn split_repo_owner_name(repo: &str) -> Result<(String, String), String> {
    let mut parts = repo.splitn(2, '/');
    let owner = parts.next().unwrap_or("").trim();
    let name = parts.next().unwrap_or("").trim();
    if owner.is_empty() || name.is_empty() {
        return Err(format!(
            "Invalid repository format '{repo}' (expected owner/name)."
        ));
    }
    Ok((owner.to_string(), name.to_string()))
}

fn extract_subissue_refs_for_parent(
    repo_owner: &str,
    repo_short_name: &str,
    parent_number: &str,
) -> Result<Vec<String>, String> {
    let output = run_gh_output(&[
        "api",
        "graphql",
        "-f",
        "query=query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){subIssues(first:100){nodes{number}}}}}",
        "-f",
        &format!("owner={repo_owner}"),
        "-f",
        &format!("name={repo_short_name}"),
        "-F",
        &format!("number={parent_number}"),
        "--jq",
        ".data.repository.issue.subIssues.nodes[]?.number | \"#\"+tostring",
    ])?;
    Ok(output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect())
}

pub(crate) fn compute_changed_files(upstream: &str) -> Result<Vec<String>, String> {
    let first = run_git_output_preserve(&["diff", "--name-only", &format!("{upstream}..HEAD")])
        .unwrap_or_default();
    if !first.trim().is_empty() {
        return Ok(parse_non_empty_lines(&first));
    }
    let fallback =
        run_git_output_preserve(&["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"])
            .unwrap_or_default();
    Ok(parse_non_empty_lines(&fallback))
}

pub(crate) fn resolve_upstream_or_default() -> String {
    run_git_output(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .unwrap_or_else(|_| "origin/dev".to_string())
}

fn parse_non_empty_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

pub(crate) fn list_staged_changed_files() -> Vec<String> {
    let staged_changed_files =
        run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRU"])
            .unwrap_or_default();
    parse_non_empty_lines(&staged_changed_files)
}

pub(crate) fn markdown_files_from(files: &[String]) -> Vec<String> {
    files
        .iter()
        .filter(|file| file.ends_with(".md"))
        .cloned()
        .collect()
}

pub(crate) fn is_docs_or_scripts_only_change(files: &[String]) -> bool {
    if files.is_empty() {
        return false;
    }
    files.iter().all(|file| {
        file.ends_with(".md")
            || file.starts_with("documentation/")
            || file.starts_with(".github/documentation/")
            || file.starts_with(".github/ISSUE_TEMPLATE/")
            || file.starts_with(".github/PULL_REQUEST_TEMPLATE/")
            || file.starts_with(".github/workflows/")
            || file.starts_with("scripts/")
    })
}

pub(crate) fn run_markdownlint_files(files: &[String]) -> Result<(), String> {
    if files.is_empty() {
        return Ok(());
    }
    let mut args = vec![
        "run".to_string(),
        "lint-md-files".to_string(),
        "--".to_string(),
    ];
    args.extend(files.iter().cloned());
    run_command_status_owned("pnpm", &args, false)
}

pub(crate) fn run_shell_syntax_checks(files: &[String]) -> Result<(), String> {
    for file in files {
        if is_shell_file_path(file) {
            run_command_status("bash", &["-n", file], false)?;
        }
    }
    Ok(())
}

pub(crate) fn is_shell_file_path(file: &str) -> bool {
    if file.ends_with(".sh") {
        return true;
    }

    let path = Path::new(file);
    if !path.is_file() {
        return false;
    }

    if !is_executable_candidate(path) {
        return false;
    }

    if let Ok(content) = fs::read_to_string(path)
        && let Some(first_line) = content.lines().next()
    {
        let line = first_line.trim();
        return line.starts_with("#!") && (line.contains("bash") || line.contains("sh"));
    }
    false
}

pub(crate) fn collect_crates_from_changed_files(
    changed_files: &str,
) -> Result<Vec<String>, String> {
    let root = repo_root()?;
    let mut crates = Vec::new();
    let mut seen = BTreeSet::new();
    for file in changed_files
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        if let Some(path) = find_crate_dir_for_file(&root, file)
            && let Some(crate_name) = super::changed_crates::read_crate_name(&root, &path)
            && seen.insert(crate_name.clone())
        {
            crates.push(crate_name);
        }
    }
    Ok(crates)
}

pub(crate) fn require_command(command: &str, install_hint: &str) -> Result<(), String> {
    if command_available(command) {
        Ok(())
    } else {
        Err(install_hint.to_string())
    }
}

pub(crate) fn ensure_git_repo() -> Result<(), String> {
    if crate::git_cli::status_success(&["rev-parse", "--is-inside-work-tree"]) {
        Ok(())
    } else {
        Err("Not a git repository.".to_string())
    }
}

pub(crate) fn repo_root() -> Result<PathBuf, String> {
    let root = run_git_output(&["rev-parse", "--show-toplevel"])?;
    if root.trim().is_empty() {
        return Err("Unable to resolve git repository root.".to_string());
    }
    Ok(PathBuf::from(root))
}

pub(crate) fn find_crate_dir_for_file(repo_root: &Path, file: &str) -> Option<String> {
    let mut cursor = repo_root.join(file);
    if !cursor.exists() {
        return None;
    }
    if cursor.is_file() {
        cursor = cursor.parent()?.to_path_buf();
    }

    loop {
        if cursor.join("Cargo.toml").is_file() {
            let relative = cursor.strip_prefix(repo_root).ok()?.to_path_buf();
            let path = relative.to_string_lossy().replace('\\', "/");
            if !path.is_empty() {
                return Some(path);
            }
        }
        if cursor == repo_root {
            break;
        }
        cursor = cursor.parent()?.to_path_buf();
    }
    None
}

pub(crate) fn command_available(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {command} >/dev/null 2>&1"))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub(crate) fn run_command_status(
    program: &str,
    args: &[&str],
    allow_failure: bool,
) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to run {program} {}: {e}", args.join(" ")))?;
    if status.success() || allow_failure {
        Ok(())
    } else {
        Err(format!(
            "{program} {} failed with exit {:?}",
            args.join(" "),
            status.code()
        ))
    }
}

pub(crate) fn run_command_status_owned(
    program: &str,
    args: &[String],
    allow_failure: bool,
) -> Result<(), String> {
    let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
    run_command_status(program, &refs, allow_failure)
}

pub(crate) fn run_git(args: &[&str]) -> Result<(), String> {
    git_cli::status(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

pub(crate) fn run_git_output(args: &[&str]) -> Result<String, String> {
    git_cli::output_trim(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

pub(crate) fn run_git_output_preserve(args: &[&str]) -> Result<String, String> {
    git_cli::output_preserve(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

pub(crate) fn git_hooks_dir(root: &Path) -> Result<PathBuf, String> {
    let output = run_git_output(&["rev-parse", "--git-path", "hooks"])?;
    let path = PathBuf::from(output.trim());
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(root.join(path))
    }
}

#[cfg(unix)]
fn is_executable_candidate(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    std::os::unix::fs::PermissionsExt::mode(&metadata.permissions()) & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_candidate(_path: &Path) -> bool {
    true
}

pub(crate) fn run_gh_status(args: &[&str]) -> Result<(), String> {
    gh_cli::status(args).map_err(|e| format!("Failed to run gh {}: {e}", args.join(" ")))
}

pub(crate) fn run_gh_output(args: &[&str]) -> Result<String, String> {
    gh_cli::output_trim(args).map_err(|e| format!("Failed to run gh {}: {e}", args.join(" ")))
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

pub(crate) fn parse_json_array(payload: &str, context: &str) -> Result<Vec<Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_array()
        .cloned()
        .ok_or_else(|| format!("Expected JSON array for {context}"))
}

fn parse_json_object(
    payload: &str,
    context: &str,
) -> Result<collections::HashMap<String, Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_object()
        .cloned()
        .ok_or_else(|| format!("Expected JSON object for {context}"))
}

pub(crate) fn object_u64(object: &collections::HashMap<String, Json>, key: &str) -> u64 {
    object.get(key).and_then(Json::as_u64).unwrap_or(0)
}

pub(crate) fn object_string(object: &collections::HashMap<String, Json>, key: &str) -> String {
    object
        .get(key)
        .and_then(Json::as_str)
        .unwrap_or_default()
        .to_string()
}

fn object_string_or_default(
    object: &collections::HashMap<String, Json>,
    key: &str,
    default: &str,
) -> String {
    let value = object_string(object, key);
    if value.trim().is_empty() {
        default.to_string()
    } else {
        value
    }
}
