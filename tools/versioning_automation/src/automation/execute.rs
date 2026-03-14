//! tools/versioning_automation/src/automation/execute.rs
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use common_json::Json;
use regex::Regex;

use crate::automation::commands::{
    AuditIssueStatusOptions, AuditSecurityOptions, AutomationAction, BuildAccountsUiOptions,
    BuildAndCheckUiBundlesOptions, BuildUiBundlesOptions, ChangedCratesOptions,
    CheckDependenciesOptions, CheckMergeConflictsOptions, CheckPriorityIssuesOptions,
    CiWatchPrOptions, CleanArtifactsOptions, InstallHooksOptions, LabelsSyncOptions,
    PostCheckoutCheckOptions, PreAddReviewOptions, PreCommitCheckOptions, PrePushCheckOptions,
    ReleasePrepareOptions, SyncMainDevCiOptions, TestCoverageOptions,
};
use crate::automation::parse::parse;
use crate::automation::render::print_usage;
use crate::repo_name::{resolve_repo_name, resolve_repo_name_optional};

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
    let result = match action {
        AutomationAction::Help => {
            print_usage();
            Ok(())
        }
        AutomationAction::AuditIssueStatus(opts) => run_audit_issue_status(opts),
        AutomationAction::AuditSecurity(opts) => run_audit_security(opts),
        AutomationAction::BuildAccountsUi(opts) => run_build_accounts_ui(opts),
        AutomationAction::BuildUiBundles(opts) => run_build_ui_bundles(opts),
        AutomationAction::BuildAndCheckUiBundles(opts) => run_build_and_check_ui_bundles(opts),
        AutomationAction::PreAddReview(opts) => run_pre_add_review(opts),
        AutomationAction::PreCommitCheck(opts) => run_pre_commit_check(opts),
        AutomationAction::PostCheckoutCheck(opts) => run_post_checkout_check(opts),
        AutomationAction::PrePushCheck(opts) => run_pre_push_check(opts),
        AutomationAction::ReleasePrepare(opts) => run_release_prepare(opts),
        AutomationAction::TestCoverage(opts) => run_test_coverage(opts),
        AutomationAction::ChangedCrates(opts) => run_changed_crates(opts),
        AutomationAction::CheckMergeConflicts(opts) => run_check_merge_conflicts(opts),
        AutomationAction::CheckDependencies(opts) => run_check_dependencies(opts),
        AutomationAction::CleanArtifacts(opts) => run_clean_artifacts(opts),
        AutomationAction::InstallHooks(opts) => run_install_hooks(opts),
        AutomationAction::CheckPriorityIssues(opts) => run_check_priority_issues(opts),
        AutomationAction::LabelsSync(opts) => run_labels_sync(opts),
        AutomationAction::CiWatchPr(opts) => run_ci_watch_pr(opts),
        AutomationAction::SyncMainDevCi(opts) => run_sync_main_dev_ci(opts),
    };

    match result {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn run_changed_crates(opts: ChangedCratesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let changed_files = git_changed_files(opts.ref1.as_deref(), opts.ref2.as_deref())?;
    if changed_files.is_empty() {
        println!("No changed files.");
        return Ok(());
    }

    let repo_root = repo_root()?;
    let mut crate_paths = BTreeSet::new();
    for file in changed_files {
        if let Some(path) = find_crate_dir_for_file(&repo_root, &file) {
            crate_paths.insert(path);
        }
    }

    if crate_paths.is_empty() {
        println!("No crates affected.");
        return Ok(());
    }

    let output_paths_only = opts.output_format.as_deref() == Some("paths");
    if output_paths_only {
        for path in crate_paths {
            println!("{path}");
        }
        return Ok(());
    }

    println!("Changed crates:");
    for path in crate_paths {
        let crate_name = read_crate_name(&repo_root, &path).unwrap_or_else(|| path.clone());
        println!("  - {crate_name} ({path})");
    }
    Ok(())
}

fn run_check_merge_conflicts(opts: CheckMergeConflictsOptions) -> Result<(), String> {
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
    let merge_base = crate::git_cli::command(&["merge-base", "HEAD", &remote_base])
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

    let check = crate::git_cli::command(&["merge-tree", &base_sha, "HEAD", &remote_base])
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

fn run_check_dependencies(opts: CheckDependenciesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    if opts.check_outdated {
        if command_available("cargo-outdated") {
            run_command_status(
                "cargo",
                &["outdated", "--workspace", "--root-deps-only"],
                true,
            )?;
        } else {
            println!("cargo-outdated not found, skipping outdated dependencies check.");
        }
    }

    run_command_status("cargo", &["check", "--workspace", "--all-targets"], false)?;

    if opts.check_unused {
        if command_available("cargo-udeps") {
            run_command_status("cargo", &["+nightly", "udeps", "--workspace"], true)?;
        } else {
            println!("cargo-udeps not found, skipping unused dependencies check.");
        }
    }
    Ok(())
}

fn run_clean_artifacts(opts: CleanArtifactsOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let root = repo_root()?;

    remove_dir_if_exists(&root.join("target"))?;
    remove_named_dirs_under(&root.join("projects"), "ui_dist")?;
    if opts.include_node_modules {
        remove_named_dirs_under(&root, "node_modules")?;
    }
    remove_nested_cargo_locks(&root.join("projects"), &root.join("Cargo.lock"))?;
    remove_files_by_suffixes(&root, &[".profraw", ".gcda", ".gcno", "~", ".bak", ".tmp"])?;

    run_command_status("cargo", &["clean"], false)?;
    println!("Build artifacts cleaned successfully.");
    Ok(())
}

fn run_install_hooks(_opts: InstallHooksOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let root = repo_root()?;
    let hooks_dir = git_hooks_dir(&root)?;
    fs::create_dir_all(&hooks_dir).map_err(|e| {
        format!(
            "Failed to create hooks directory '{}': {e}",
            hooks_dir.display()
        )
    })?;

    println!("Installing hooks to '{}'", hooks_dir.display());

    write_hook_script(
        &hooks_dir.join("pre-commit"),
        PRE_COMMIT_HOOK_SCRIPT,
        "pre-commit",
    )?;
    copy_tracked_hook(
        &root,
        &hooks_dir,
        "scripts/automation/git_hooks/prepare-commit-msg",
        "prepare-commit-msg",
    )?;
    copy_tracked_hook(
        &root,
        &hooks_dir,
        "scripts/automation/git_hooks/commit-msg",
        "commit-msg",
    )?;
    write_hook_script(
        &hooks_dir.join("pre-push"),
        PRE_PUSH_HOOK_SCRIPT,
        "pre-push",
    )?;
    write_hook_script(
        &hooks_dir.join("post-checkout"),
        POST_CHECKOUT_HOOK_SCRIPT,
        "post-checkout",
    )?;
    copy_tracked_hook(
        &root,
        &hooks_dir,
        "scripts/automation/git_hooks/pre-branch-create",
        "pre-branch-create",
    )?;
    write_hook_script(
        &hooks_dir.join("branch-creation-check"),
        BRANCH_CREATION_CHECK_SCRIPT,
        "branch-creation-check",
    )?;

    println!("Hooks installed successfully.");
    Ok(())
}

fn run_audit_security(_opts: AuditSecurityOptions) -> Result<(), String> {
    ensure_git_repo()?;
    if !command_available("cargo-audit") {
        return Err("cargo-audit not found. Install with: cargo install cargo-audit".to_string());
    }
    let _ = run_command_status("cargo", &["audit", "fetch"], true);
    run_command_status("cargo", &["audit"], false)
}

fn run_build_accounts_ui(_opts: BuildAccountsUiOptions) -> Result<(), String> {
    ensure_git_repo()?;
    require_command(
        "dx",
        "dx (dioxus-cli) not found. Install with: cargo install dioxus-cli",
    )?;
    let root = repo_root()?;
    let ui_dir = root.join("projects/products/stable/accounts/ui");
    if !ui_dir.is_dir() {
        return Err(format!("UI directory not found: '{}'", ui_dir.display()));
    }
    build_ui_bundle_for_dir(&ui_dir)?;
    println!(
        "Accounts UI bundle generated in {}",
        ui_dir.join("ui_dist").display()
    );
    Ok(())
}

fn run_build_ui_bundles(_opts: BuildUiBundlesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    require_command(
        "dx",
        "dx (dioxus-cli) not found. Install with: cargo install dioxus-cli",
    )?;
    let root = repo_root()?;
    let ui_dirs = find_ui_dirs(&root.join("projects/products"))?;
    if ui_dirs.is_empty() {
        return Err("No UI crates found under projects/products".to_string());
    }

    for ui_dir in ui_dirs {
        let cargo_toml = ui_dir.join("Cargo.toml");
        if !cargo_contains_dioxus(&cargo_toml)? {
            println!("Skipping {} (no dioxus dependency)", ui_dir.display());
            continue;
        }
        println!("Building UI bundle in {}", ui_dir.display());
        build_ui_bundle_for_dir(&ui_dir)?;
    }
    println!("UI bundle build complete");
    Ok(())
}

fn run_build_and_check_ui_bundles(_opts: BuildAndCheckUiBundlesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    require_command(
        "dx",
        "dx (dioxus-cli) not found. Install with: cargo install dioxus-cli",
    )?;
    let root = repo_root()?;
    let ui_dirs = find_ui_dirs(&root.join("projects/products"))?;
    if ui_dirs.is_empty() {
        return Err("No UI crates found under projects/products".to_string());
    }

    let mut missing = Vec::new();
    for ui_dir in ui_dirs {
        let cargo_toml = ui_dir.join("Cargo.toml");
        if !cargo_contains_dioxus(&cargo_toml)? {
            println!("Skipping {} (no dioxus dependency)", ui_dir.display());
            continue;
        }
        println!("Building UI bundle in {}", ui_dir.display());
        build_ui_bundle_for_dir(&ui_dir)?;
        if !ui_bundle_artifacts_ok(&ui_dir.join("ui_dist"))? {
            missing.push(ui_dir.to_string_lossy().to_string());
        }
    }

    if missing.is_empty() {
        println!("UI bundle build + check complete");
        return Ok(());
    }

    eprintln!("Missing UI bundle artifacts in:");
    for path in missing {
        eprintln!(" - {path}");
    }
    Err("One or more UI bundles are incomplete.".to_string())
}

fn run_pre_add_review(_opts: PreAddReviewOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let mut issues = 0u32;

    println!("Running pre-add review...");

    println!("Checking code formatting...");
    if command_status_success("cargo", &["fmt", "--all", "--check"])? {
        println!("OK: code is properly formatted.");
    } else {
        eprintln!("Formatting issues detected. Run: cargo fmt");
        issues += 1;
    }

    println!("Running clippy...");
    if command_status_success(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )? {
        println!("OK: no clippy warnings.");
    } else {
        eprintln!("Clippy warnings or errors detected.");
        issues += 1;
    }

    println!("Running tests...");
    if command_status_success("cargo", &["test", "--workspace"])? {
        println!("OK: all tests passed.");
    } else {
        eprintln!("Some tests failed.");
        issues += 1;
    }

    println!("Checking staged changes for risky patterns...");
    let staged_patch = run_git_output_preserve(&["diff", "--cached", "--unified=0"])?;
    let risky_patterns = ["unwrap(", "expect(", "todo!", "unimplemented!", "panic!"];
    let mut found_patterns = 0u32;
    for pattern in risky_patterns {
        if staged_patch
            .lines()
            .any(|line| line.starts_with('+') && !line.starts_with("+++") && line.contains(pattern))
        {
            eprintln!("Found '{}' in staged changes.", pattern);
            found_patterns += 1;
        }
    }
    if found_patterns > 0 {
        issues += 1;
    }

    println!("Summarizing touched crates...");
    let staged_files =
        run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRU"])?;
    let root = repo_root()?;
    let mut crates = BTreeSet::new();
    for file in staged_files
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        if let Some(path) = find_crate_dir_for_file(&root, file) {
            crates.insert(path);
        }
    }
    if crates.is_empty() {
        println!("No crates touched.");
    } else {
        println!("Touched crates:");
        for path in crates {
            println!("  - {path}");
        }
    }

    if issues == 0 {
        println!("Pre-add review passed.");
        Ok(())
    } else {
        Err(format!(
            "Pre-add review found {issues} issue(s). Please review before staging."
        ))
    }
}

fn run_post_checkout_check(_opts: PostCheckoutCheckOptions) -> Result<(), String> {
    let upstream = run_git_output(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .unwrap_or_default();
    let upstream_branch = if upstream.trim().is_empty() {
        "origin/dev".to_string()
    } else {
        upstream.trim().to_string()
    };

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

fn run_pre_commit_check(_opts: PreCommitCheckOptions) -> Result<(), String> {
    if std::env::var("SKIP_PRE_COMMIT").unwrap_or_default() == "1" {
        println!("⚠️  Pre-commit checks skipped (SKIP_PRE_COMMIT=1)");
        return Ok(());
    }

    println!("📝 Running pre-commit checks...");
    println!();
    ensure_git_repo()?;

    let current_branch = run_git_output(&["rev-parse", "--abbrev-ref", "HEAD"])?;
    if std::env::var("ALLOW_PROTECTED_BRANCH_COMMIT").unwrap_or_default() != "1"
        && (current_branch.trim() == "dev" || current_branch.trim() == "main")
    {
        return Err(format!(
            "❌ Direct commits on protected branch '{}' are blocked.\n   Create a feature/fix/docs branch, then open a PR.\n   Temporary bypass (exception only): ALLOW_PROTECTED_BRANCH_COMMIT=1 git commit ...",
            current_branch.trim()
        ));
    }

    let upstream = run_git_output(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .unwrap_or_else(|_| "origin/dev".to_string());
    let push_commits =
        run_git_output_preserve(&["log", &format!("{upstream}..HEAD"), "--format=%B"])
            .unwrap_or_default();
    if !push_commits.trim().is_empty() {
        validate_part_of_only_policy(&push_commits, resolve_repo_name(None).ok().as_deref())
            .map_err(|err| {
                format!("{err}\n\n❌ Assignment policy check failed (early pre-commit guard).")
            })?;
    }

    let staged_changed_files =
        run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRU"])
            .unwrap_or_default();
    let staged_files = staged_changed_files
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let crates = collect_crates_from_changed_files(&staged_changed_files).unwrap_or_default();
    if crates.is_empty() {
        println!("🎯 No Rust crates detected, checking all files");
    } else {
        println!("🎯 Affected crates:");
        for crate_name in &crates {
            println!("   - {crate_name}");
        }
        println!();
    }

    let markdown_files = staged_files
        .iter()
        .filter(|file| file.ends_with(".md"))
        .cloned()
        .collect::<Vec<_>>();
    if markdown_files.is_empty() {
        println!("📝 Skipping markdown lint (no staged markdown files)");
    } else {
        println!("📝 Auto-fixing markdown files...");
        if let Err(err) = run_markdownlint_files(&markdown_files) {
            return Err(format!(
                "{err}\n\n❌ Markdown lint failed on staged markdown files."
            ));
        }
    }

    println!("🔎 Checking shell syntax...");
    for file in &staged_files {
        if is_shell_file_path(file) {
            if let Err(err) = run_command_status("bash", &["-n", file], false) {
                return Err(format!(
                    "   ❌ Shell syntax error: {file}\n{err}\n\n❌ Shell syntax checks failed!"
                ));
            }
        }
    }

    if staged_files.iter().any(|file| file.ends_with(".rs")) {
        println!("✨ Formatting code...");
        run_command_status("cargo", &["fmt", "--all"], false)?;
    } else {
        println!("✨ Skipping Rust formatting (no staged Rust files)");
    }

    let restage_files =
        run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRU"])
            .unwrap_or_default()
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
    if !restage_files.is_empty() {
        let mut args = vec!["add".to_string(), "--".to_string()];
        args.extend(restage_files);
        run_command_status_owned("git", &args, false)?;
    }

    println!("✅ Pre-commit checks passed");
    println!();
    Ok(())
}

fn run_pre_push_check(_opts: PrePushCheckOptions) -> Result<(), String> {
    if std::env::var("SKIP_PRE_PUSH").unwrap_or_default() == "1" {
        println!("Pre-push checks skipped (SKIP_PRE_PUSH=1)");
        return Ok(());
    }
    ensure_git_repo()?;
    let upstream = run_git_output(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .unwrap_or_else(|_| "origin/dev".to_string());
    let commits = run_git_output_preserve(&["log", &format!("{upstream}..HEAD"), "--format=%B"])
        .unwrap_or_default();
    let repo = resolve_repo_name(None).ok();

    if !commits.trim().is_empty() {
        validate_no_root_parent_refs(&commits, repo.as_deref())?;
        validate_part_of_only_policy(&commits, repo.as_deref())?;
    }

    let changed_files = compute_changed_files(&upstream)?;
    let markdown_files = changed_files
        .iter()
        .filter(|f| f.ends_with(".md"))
        .cloned()
        .collect::<Vec<_>>();
    let docs_or_scripts_only = is_docs_or_scripts_only_change(&changed_files);

    if docs_or_scripts_only {
        if !markdown_files.is_empty() {
            run_markdownlint_files(&markdown_files)?;
        }
        run_shell_syntax_checks(&changed_files)?;
        println!("Pre-push checks passed (docs/scripts-only mode)");
        return Ok(());
    }

    if !markdown_files.is_empty() {
        run_markdownlint_files(&markdown_files)?;
    }
    run_command_status("cargo", &["fmt", "--all", "--", "--check"], false)?;

    let changed_file_text = changed_files.join("\n");
    let mut crates = collect_crates_from_changed_files(&changed_file_text)?;
    crates.sort();
    crates.dedup();

    let has_lockfile = repo_root()?.join("Cargo.lock").is_file();
    let mut clippy_args = Vec::<String>::new();
    let mut test_args = Vec::<String>::new();
    if has_lockfile {
        clippy_args.push("--locked".to_string());
        test_args.push("--locked".to_string());
    }
    if crates.is_empty() {
        clippy_args.extend(["--workspace", "--all-targets", "--all-features"].map(String::from));
        test_args.extend(["--workspace", "--all-targets", "--all-features"].map(String::from));
    } else {
        clippy_args.extend(["--all-targets", "--all-features"].map(String::from));
        test_args.extend(["--all-targets", "--all-features"].map(String::from));
        for crate_name in crates {
            clippy_args.push("-p".to_string());
            clippy_args.push(crate_name.clone());
            test_args.push("-p".to_string());
            test_args.push(crate_name);
        }
    }

    let mut clippy_run = vec!["clippy".to_string()];
    clippy_run.extend(clippy_args);
    clippy_run.push("--".to_string());
    clippy_run.push("-D".to_string());
    clippy_run.push("warnings".to_string());
    run_command_status_owned("cargo", &clippy_run, false)?;

    let mut test_run = vec!["test".to_string()];
    test_run.extend(test_args);
    run_command_status_owned("cargo", &test_run, false)?;

    println!("All pre-push checks passed");
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

fn run_audit_issue_status(opts: AuditIssueStatusOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let repo = resolve_repo_name(opts.repo).map_err(|e| e.to_string())?;
    let range = format!("{}..{}", opts.base_ref, opts.head_ref);

    let open_issues_json = run_gh_output(&[
        "issue",
        "list",
        "--state",
        "open",
        "--limit",
        &opts.limit.to_string(),
        "--json",
        "number,title,url,body,labels,state",
        "-R",
        &repo,
    ])?;
    let open_issues = parse_json_array(&open_issues_json, "open issues JSON")?;
    let total_open = open_issues.len();

    let commit_messages = run_git_output_preserve(&["log", &range, "--format=%B"])?;
    let (closing_refs, part_refs) = extract_issue_refs_from_text(&commit_messages)?;

    let mut would_close_items = Vec::new();
    let mut part_only_items = Vec::new();
    let mut unreferenced_items = Vec::new();
    let mut done_in_dev_items = Vec::new();

    for issue in open_issues {
        let Some(obj) = issue.as_object() else {
            continue;
        };
        let number = object_u64(obj, "number");
        if number == 0 {
            continue;
        }
        let issue_id = number.to_string();
        let title = object_string(obj, "title");
        let url = object_string(obj, "url");
        let body = object_string(obj, "body");
        let parent = extract_parent_field(&body).unwrap_or_else(|| "(none)".to_string());

        let labels_csv = obj
            .get("labels")
            .and_then(Json::as_array)
            .map(|labels| {
                labels
                    .iter()
                    .filter_map(|label| label.as_object())
                    .map(|label_obj| object_string(label_obj, "name").to_lowercase())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default();

        let line = format!("- [#{issue_id}]({url}) {title} (parent: {parent})");
        if labels_csv.contains("done-in-dev") {
            done_in_dev_items.push(line);
        } else if closing_refs.contains(&issue_id) {
            would_close_items.push(line);
        } else if part_refs.contains(&issue_id) {
            part_only_items.push(line);
        } else {
            unreferenced_items.push(line);
        }
    }

    let report = render_issue_audit_report(
        &repo,
        &range,
        total_open,
        &done_in_dev_items,
        &would_close_items,
        &part_only_items,
        &unreferenced_items,
    );

    if let Some(output_file) = opts.output_file {
        fs::write(&output_file, &report)
            .map_err(|e| format!("Failed to write report to '{}': {e}", output_file))?;
        println!("Generated file: {output_file}");
    }
    print!("{report}");
    Ok(())
}

fn run_release_prepare(opts: ReleasePrepareOptions) -> Result<(), String> {
    ensure_git_repo()?;
    require_clean_tree()?;
    validate_semver(&opts.version)?;

    let current_branch = run_git_output(&["branch", "--show-current"])?;
    if current_branch.trim() != "main" {
        println!(
            "Warning: current branch is '{}', not 'main'.",
            current_branch.trim()
        );
    }

    run_command_status("cargo", &["test", "--workspace"], false)?;

    if command_available("cargo-audit") {
        run_command_status("cargo", &["audit"], false)?;
    }

    let root = repo_root()?;
    let root_cargo = root.join("Cargo.toml");
    if root_cargo.is_file() {
        update_version_in_cargo_file(&root_cargo, &opts.version)?;
    }

    let mut project_cargos = Vec::new();
    collect_files_named(&root.join("projects"), "Cargo.toml", &mut project_cargos)?;
    for cargo_toml in project_cargos {
        update_version_in_cargo_file(&cargo_toml, &opts.version)?;
    }

    let changelog_path = root.join("CHANGELOG.md");
    if opts.auto_changelog {
        update_changelog(&changelog_path, &opts.version)?;
    } else {
        println!("Skipping automatic changelog generation.");
    }

    run_git(&["add", "-u"])?;
    let commit_message = format!(
        "chore: prepare release v{}\n\nRelease preparation for version {}.\n",
        opts.version, opts.version
    );
    run_git(&["commit", "-m", &commit_message])?;
    let tag_name = format!("v{}", opts.version);
    run_git(&[
        "tag",
        "-a",
        &tag_name,
        "-m",
        &format!("Release {}", tag_name),
    ])?;
    println!("Release {} prepared.", tag_name);
    Ok(())
}

fn run_check_priority_issues(opts: CheckPriorityIssuesOptions) -> Result<(), String> {
    let mut by_number: BTreeMap<u64, (String, String)> = BTreeMap::new();
    for label in ["high priority", "security"] {
        let mut args = vec![
            "issue",
            "list",
            "--state",
            "open",
            "--limit",
            "200",
            "--label",
            label,
            "--json",
            "number,title,url",
        ];
        if let Some(repo) = opts.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        let output = run_gh_output(&args)?;
        let issues = parse_json_array(&output, "issues JSON")?;
        for issue in issues {
            let Some(issue_object) = issue.as_object() else {
                continue;
            };
            let number = object_u64(issue_object, "number");
            if number == 0 {
                continue;
            }
            by_number.insert(
                number,
                (
                    object_string(issue_object, "title"),
                    object_string(issue_object, "url"),
                ),
            );
        }
    }

    if by_number.is_empty() {
        println!("No high priority or security issues found.");
        return Ok(());
    }

    println!("HIGH PRIORITY & SECURITY ISSUES");
    println!();
    for (idx, (number, (title, url))) in by_number.iter().enumerate() {
        println!("[{}] Issue #{}", idx + 1, number);
        println!("    Title: {}", title);
        println!("    URL:   {}", url);
        println!();
    }
    println!("Total priority issues: {}", by_number.len());

    Ok(())
}

fn run_labels_sync(opts: LabelsSyncOptions) -> Result<(), String> {
    let content = fs::read_to_string(&opts.labels_file).map_err(|e| {
        format!(
            "Labels file not found or unreadable '{}': {e}",
            opts.labels_file
        )
    })?;
    let labels = parse_labels(&content, &opts.labels_file)?;

    let existing = run_gh_output(&[
        "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
    ])?;
    let mut existing_set: BTreeSet<String> = existing
        .lines()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect();

    for (name, color, description) in &labels {
        if name.trim().is_empty() {
            return Err("Label missing field 'name'".to_string());
        }
        if color.trim().is_empty() {
            return Err(format!("Label '{name}' missing field 'color'"));
        }

        if existing_set.contains(name) {
            run_gh_status(&[
                "label",
                "edit",
                name,
                "--color",
                color,
                "--description",
                description,
            ])?;
        } else {
            run_gh_status(&[
                "label",
                "create",
                name,
                "--color",
                color,
                "--description",
                description,
            ])?;
            existing_set.insert(name.clone());
        }
    }

    if opts.prune {
        let desired: BTreeSet<String> = labels
            .iter()
            .map(|(name, _, _)| name.clone())
            .filter(|name| !name.trim().is_empty())
            .collect();

        let repo_labels = run_gh_output(&[
            "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
        ])?;
        let protected: BTreeSet<String> = [
            "bug",
            "documentation",
            "duplicate",
            "enhancement",
            "good first issue",
            "help wanted",
            "invalid",
            "question",
            "wontfix",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        for label in repo_labels
            .lines()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
        {
            if desired.contains(label) || protected.contains(label) {
                continue;
            }
            let _ = run_gh_status(&["label", "delete", label, "--yes"]);
        }
    }

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

fn require_clean_tree() -> Result<(), String> {
    let unstaged_clean = crate::git_cli::status_success(&["diff", "--quiet"]);
    let staged_clean = crate::git_cli::status_success(&["diff", "--cached", "--quiet"]);
    if unstaged_clean && staged_clean {
        Ok(())
    } else {
        Err("Working tree is dirty. Commit/stash your changes first.".to_string())
    }
}

fn validate_semver(version: &str) -> Result<(), String> {
    let re = Regex::new(r"^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$")
        .map_err(|e| format!("Failed to compile semver regex: {e}"))?;
    if re.is_match(version) {
        Ok(())
    } else {
        Err(format!(
            "Invalid version format: {version}. Expected semver format."
        ))
    }
}

fn update_version_in_cargo_file(path: &Path, version: &str) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
    let mut changed = false;
    let updated = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("version = \"") {
                changed = true;
                format!("version = \"{version}\"")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    if changed {
        fs::write(path, format!("{updated}\n"))
            .map_err(|e| format!("Failed to write '{}': {e}", path.display()))?;
    }
    Ok(())
}

fn collect_files_named(root: &Path, file_name: &str, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", root.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            collect_files_named(&path, file_name, out)?;
            continue;
        }
        if file_type.is_file() && path.file_name().and_then(|v| v.to_str()) == Some(file_name) {
            out.push(path);
        }
    }
    Ok(())
}

fn update_changelog(path: &Path, version: &str) -> Result<(), String> {
    let today = run_command_capture("date", &["+%Y-%m-%d"])?;
    let last_tag = run_git_output(&["describe", "--tags", "--abbrev=0"]).unwrap_or_default();
    let commits = if last_tag.trim().is_empty() {
        run_git_output_preserve(&["log", "--oneline", "--no-merges"])?
    } else {
        run_git_output_preserve(&[
            "log",
            &format!("{}..HEAD", last_tag.trim()),
            "--oneline",
            "--no-merges",
        ])?
    };
    let mut lines = vec![
        "# Changelog".to_string(),
        "".to_string(),
        format!("## [v{version}] - {}", today.trim()),
        "".to_string(),
        "### Changes".to_string(),
        "".to_string(),
    ];
    lines.extend(
        commits
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| format!("- {line}")),
    );
    lines.push("".to_string());

    if path.is_file() {
        let existing = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
        let mut existing_lines = existing.lines();
        let _ = existing_lines.next();
        lines.extend(existing_lines.map(ToString::to_string));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))
        .map_err(|e| format!("Failed to write '{}': {e}", path.display()))?;
    Ok(())
}

fn run_command_capture(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run {program} {}: {e}", args.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "{program} {} failed with exit {:?}",
            args.join(" "),
            output.status.code()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn extract_issue_refs_from_text(
    text: &str,
) -> Result<(BTreeSet<String>, BTreeSet<String>), String> {
    let re = Regex::new(
        r"(?i)(closes|fixes|resolves|part\s+of|related\s+to|reopen|reopens)\s+#([0-9]+)",
    )
    .map_err(|e| format!("Failed to compile refs regex: {e}"))?;
    let mut closing = BTreeSet::new();
    let mut part = BTreeSet::new();
    for cap in re.captures_iter(text) {
        let keyword = cap
            .get(1)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        let issue_number = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        if issue_number.is_empty() {
            continue;
        }
        if keyword == "closes" || keyword == "fixes" || keyword == "resolves" {
            closing.insert(issue_number);
        } else {
            part.insert(issue_number);
        }
    }
    Ok((closing, part))
}

fn extract_parent_field(body: &str) -> Option<String> {
    let re =
        Regex::new(r"(?i)^\s*Parent:\s*(#?[0-9]+|none|base|epic|\(none\)|\(base\)|\(epic\))\s*$")
            .ok()?;
    let mut parent_value: Option<String> = None;
    for line in body.lines() {
        if let Some(cap) = re.captures(line) {
            parent_value = cap.get(1).map(|m| m.as_str().trim().to_lowercase());
        }
    }
    parent_value.map(|raw| {
        raw.trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string()
    })
}

fn render_issue_audit_report(
    repo: &str,
    range: &str,
    total_open: usize,
    done_in_dev_items: &[String],
    would_close_items: &[String],
    part_only_items: &[String],
    unreferenced_items: &[String],
) -> String {
    let mut out = Vec::new();
    out.push("# Issue Status Audit".to_string());
    out.push("".to_string());
    out.push(format!("- Repository: `{repo}`"));
    out.push(format!("- Range: `{range}`"));
    out.push("".to_string());
    out.push("## Summary".to_string());
    out.push("".to_string());
    out.push(format!("- Open issues fetched: {total_open}"));
    out.push(format!(
        "- Would close on merge: {}",
        would_close_items.len()
    ));
    out.push(format!(
        "- Done in dev (label): {}",
        done_in_dev_items.len()
    ));
    out.push(format!(
        "- Part-of-only (not closing): {}",
        part_only_items.len()
    ));
    out.push(format!(
        "- Unreferenced in range: {}",
        unreferenced_items.len()
    ));
    out.push("".to_string());
    out.push("## Done In Dev (Label)".to_string());
    out.push("".to_string());
    if done_in_dev_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(done_in_dev_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Would Close On Merge".to_string());
    out.push("".to_string());
    if would_close_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(would_close_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Part-Of Only".to_string());
    out.push("".to_string());
    if part_only_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(part_only_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Unreferenced".to_string());
    out.push("".to_string());
    if unreferenced_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(unreferenced_items.iter().cloned());
    }
    out.push("".to_string());
    out.join("\n")
}

fn validate_no_root_parent_refs(commits: &str, repo: Option<&str>) -> Result<(), String> {
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

fn validate_part_of_only_policy(commits: &str, repo: Option<&str>) -> Result<(), String> {
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
    let re = Regex::new(
        r"(?i)(closes|fixes|resolves|part\s+of|related\s+to|reopen|reopens)\s+#([0-9]+)",
    )
    .map_err(|e| format!("Failed to compile refs regex: {e}"))?;
    let mut out = Vec::new();
    for cap in re.captures_iter(text) {
        let action = cap
            .get(1)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        let issue = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        if !issue.is_empty() {
            out.push((action, issue));
        }
    }
    Ok(out)
}

fn extract_issue_refs_hook_detailed(text: &str) -> Result<Vec<(String, String)>, String> {
    let re = Regex::new(r"(?i)(closes|fixes|part\s+of|reopen|reopens)\s+#([0-9]+)")
        .map_err(|e| format!("Failed to compile refs regex: {e}"))?;
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for cap in re.captures_iter(text) {
        let action = cap
            .get(1)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        let issue = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        if !issue.is_empty() {
            let key = format!("{action}|{issue}");
            if seen.insert(key) {
                out.push((action, issue));
            }
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
    let parent = extract_parent_field(&body)
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

fn compute_changed_files(upstream: &str) -> Result<Vec<String>, String> {
    let first = run_git_output_preserve(&["diff", "--name-only", &format!("{upstream}..HEAD")])
        .unwrap_or_default();
    if !first.trim().is_empty() {
        return Ok(first
            .lines()
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(ToString::to_string)
            .collect());
    }
    let fallback =
        run_git_output_preserve(&["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"])
            .unwrap_or_default();
    Ok(fallback
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
        .collect())
}

fn is_docs_or_scripts_only_change(files: &[String]) -> bool {
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

fn run_markdownlint_files(files: &[String]) -> Result<(), String> {
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

fn run_shell_syntax_checks(files: &[String]) -> Result<(), String> {
    for file in files {
        if is_shell_file_path(file) {
            run_command_status("bash", &["-n", file], false)?;
        }
    }
    Ok(())
}

fn is_shell_file_path(file: &str) -> bool {
    if file.ends_with(".sh") {
        return true;
    }

    let path = Path::new(file);
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let Ok(metadata) = fs::metadata(path) else {
            return false;
        };
        if metadata.permissions().mode() & 0o111 == 0 {
            return false;
        }
    }

    if let Ok(content) = fs::read_to_string(path)
        && let Some(first_line) = content.lines().next()
    {
        let line = first_line.trim();
        return line.starts_with("#!") && (line.contains("bash") || line.contains("sh"));
    }
    false
}

fn collect_crates_from_changed_files(changed_files: &str) -> Result<Vec<String>, String> {
    let root = repo_root()?;
    let mut crates = Vec::new();
    let mut seen = BTreeSet::new();
    for file in changed_files
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        if let Some(path) = find_crate_dir_for_file(&root, file)
            && let Some(crate_name) = read_crate_name(&root, &path)
            && seen.insert(crate_name.clone())
        {
            crates.push(crate_name);
        }
    }
    Ok(crates)
}

fn require_command(command: &str, install_hint: &str) -> Result<(), String> {
    if command_available(command) {
        Ok(())
    } else {
        Err(install_hint.to_string())
    }
}

fn find_ui_dirs(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = Vec::new();
    find_ui_dirs_recursive(root, &mut dirs)?;
    dirs.sort();
    Ok(dirs)
}

fn find_ui_dirs_recursive(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", root.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }

        if path.file_name().and_then(|v| v.to_str()) == Some("ui")
            && path.join("Cargo.toml").is_file()
        {
            out.push(path);
            continue;
        }
        find_ui_dirs_recursive(&path, out)?;
    }
    Ok(())
}

fn cargo_contains_dioxus(cargo_toml: &Path) -> Result<bool, String> {
    let content = fs::read_to_string(cargo_toml)
        .map_err(|e| format!("Failed to read '{}': {e}", cargo_toml.display()))?;
    Ok(content.contains("dioxus"))
}

fn build_ui_bundle_for_dir(ui_dir: &Path) -> Result<(), String> {
    let mut cmd = Command::new("dx");
    cmd.arg("bundle")
        .arg("--release")
        .arg("--debug-symbols")
        .arg("false")
        .arg("--out-dir")
        .arg("ui_dist")
        .current_dir(ui_dir);
    let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
    let merged_rustflags = if rustflags.trim().is_empty() {
        "-C debuginfo=0".to_string()
    } else {
        format!("{rustflags} -C debuginfo=0")
    };
    cmd.env("CARGO_PROFILE_RELEASE_DEBUG", "0");
    cmd.env("RUSTFLAGS", merged_rustflags);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run dx bundle in '{}': {e}", ui_dir.display()))?;
    if !status.success() {
        return Err(format!(
            "dx bundle failed in '{}' with exit {:?}",
            ui_dir.display(),
            status.code()
        ));
    }

    let manifest = ui_dir.join("ui_manifest.ron");
    let out_manifest = ui_dir.join("ui_dist").join("ui_manifest.ron");
    if manifest.is_file() {
        fs::copy(&manifest, &out_manifest).map_err(|e| {
            format!(
                "Failed to copy '{}' to '{}': {e}",
                manifest.display(),
                out_manifest.display()
            )
        })?;
    }
    Ok(())
}

fn ui_bundle_artifacts_ok(ui_dist: &Path) -> Result<bool, String> {
    let index = ui_dist.join("public/index.html");
    if !index.is_file() {
        return Ok(false);
    }
    let assets = ui_dist.join("public/assets");
    let mut has_js = false;
    let mut has_wasm = false;
    let entries = match fs::read_dir(&assets) {
        Ok(entries) => entries,
        Err(_) => return Ok(false),
    };
    for entry in entries {
        let entry = entry
            .map_err(|e| format!("Failed to read assets under '{}': {e}", assets.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_file() {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        if file_name.ends_with(".js") {
            has_js = true;
        }
        if file_name.ends_with(".wasm") {
            has_wasm = true;
        }
    }
    Ok(has_js && has_wasm)
}

fn ensure_git_repo() -> Result<(), String> {
    if crate::git_cli::status_success(&["rev-parse", "--is-inside-work-tree"]) {
        Ok(())
    } else {
        Err("Not a git repository.".to_string())
    }
}

fn repo_root() -> Result<PathBuf, String> {
    let root = run_git_output(&["rev-parse", "--show-toplevel"])?;
    if root.trim().is_empty() {
        return Err("Unable to resolve git repository root.".to_string());
    }
    Ok(PathBuf::from(root))
}

fn git_changed_files(ref1: Option<&str>, ref2: Option<&str>) -> Result<Vec<String>, String> {
    let output = match (ref1, ref2) {
        (Some(a), Some(b)) => run_git_output_preserve(&["diff", "--name-only", a, b])?,
        (Some(a), None) => run_git_output_preserve(&["diff", "--name-only", a, "HEAD"])?,
        (None, None) => run_git_output_preserve(&["diff", "--name-only", "HEAD"])?,
        (None, Some(_)) => {
            return Err("Second ref provided without first ref.".to_string());
        }
    };
    Ok(output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect())
}

fn find_crate_dir_for_file(repo_root: &Path, file: &str) -> Option<String> {
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

fn read_crate_name(repo_root: &Path, crate_path: &str) -> Option<String> {
    let cargo_toml = repo_root.join(crate_path).join("Cargo.toml");
    let content = fs::read_to_string(cargo_toml).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("name") {
            let rhs = rest.trim_start();
            if let Some(value) = rhs.strip_prefix('=') {
                let raw = value.trim();
                if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
                    return Some(raw[1..raw.len() - 1].to_string());
                }
            }
        }
    }
    None
}

fn command_available(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {command} >/dev/null 2>&1"))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run_command_status(program: &str, args: &[&str], allow_failure: bool) -> Result<(), String> {
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

fn run_command_status_owned(
    program: &str,
    args: &[String],
    allow_failure: bool,
) -> Result<(), String> {
    let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
    run_command_status(program, &refs, allow_failure)
}

fn command_status_success(program: &str, args: &[&str]) -> Result<bool, String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to run {program} {}: {e}", args.join(" ")))?;
    Ok(status.success())
}

fn remove_dir_if_exists(path: &Path) -> Result<(), String> {
    if path.is_dir() {
        fs::remove_dir_all(path)
            .map_err(|e| format!("Failed to remove directory '{}': {e}", path.display()))?;
    }
    Ok(())
}

fn remove_named_dirs_under(root: &Path, target_name: &str) -> Result<(), String> {
    if !root.is_dir() {
        return Ok(());
    }
    remove_named_dirs_recursive(root, target_name)
}

fn remove_named_dirs_recursive(dir: &Path, target_name: &str) -> Result<(), String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", dir.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }
        if path.file_name().and_then(|v| v.to_str()) == Some(target_name) {
            fs::remove_dir_all(&path)
                .map_err(|e| format!("Failed to remove directory '{}': {e}", path.display()))?;
            continue;
        }
        remove_named_dirs_recursive(&path, target_name)?;
    }
    Ok(())
}

fn remove_nested_cargo_locks(projects_root: &Path, root_lock: &Path) -> Result<(), String> {
    if !projects_root.is_dir() {
        return Ok(());
    }
    remove_nested_cargo_locks_recursive(projects_root, root_lock)
}

fn remove_nested_cargo_locks_recursive(dir: &Path, root_lock: &Path) -> Result<(), String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", dir.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            remove_nested_cargo_locks_recursive(&path, root_lock)?;
            continue;
        }
        if file_type.is_file()
            && path.file_name().and_then(|v| v.to_str()) == Some("Cargo.lock")
            && path != root_lock
        {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed to remove file '{}': {e}", path.display()))?;
        }
    }
    Ok(())
}

fn remove_files_by_suffixes(root: &Path, suffixes: &[&str]) -> Result<(), String> {
    if !root.is_dir() {
        return Ok(());
    }
    remove_files_by_suffixes_recursive(root, suffixes)
}

fn remove_files_by_suffixes_recursive(dir: &Path, suffixes: &[&str]) -> Result<(), String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", dir.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            remove_files_by_suffixes_recursive(&path, suffixes)?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let Some(path_text) = path.to_str() else {
            continue;
        };
        if suffixes.iter().any(|suffix| path_text.ends_with(suffix)) {
            let _ = fs::remove_file(&path);
        }
    }
    Ok(())
}

fn run_git(args: &[&str]) -> Result<(), String> {
    crate::git_cli::status(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

fn run_git_output(args: &[&str]) -> Result<String, String> {
    crate::git_cli::output_trim(args)
        .map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

fn run_git_output_preserve(args: &[&str]) -> Result<String, String> {
    crate::git_cli::output_preserve(args)
        .map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

fn git_hooks_dir(root: &Path) -> Result<PathBuf, String> {
    let output = run_git_output(&["rev-parse", "--git-path", "hooks"])?;
    let path = PathBuf::from(output.trim());
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(root.join(path))
    }
}

fn copy_tracked_hook(
    root: &Path,
    hooks_dir: &Path,
    source_relative: &str,
    hook_name: &str,
) -> Result<(), String> {
    let source = root.join(source_relative);
    if !source.is_file() {
        return Err(format!("Missing hook source '{}'", source.display()));
    }
    let destination = hooks_dir.join(hook_name);
    fs::copy(&source, &destination).map_err(|e| {
        format!(
            "Failed to copy '{}' to '{}': {e}",
            source.display(),
            destination.display()
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&destination)
            .map_err(|e| format!("Failed to read '{}': {e}", destination.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&destination, perms)
            .map_err(|e| format!("Failed to chmod '{}': {e}", destination.display()))?;
    }
    println!("✅ Installed {hook_name}");
    Ok(())
}

fn write_hook_script(path: &Path, content: &str, hook_name: &str) -> Result<(), String> {
    fs::write(path, content)
        .map_err(|e| format!("Failed to write hook '{}': {e}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .map_err(|e| format!("Failed to chmod '{}': {e}", path.display()))?;
    }
    println!("✅ Installed {hook_name}");
    Ok(())
}

const PRE_COMMIT_HOOK_SCRIPT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
if ! command -v versioning_automation >/dev/null 2>&1; then
	echo "Error: command 'versioning_automation' not found." >&2
	exit 127
fi
exec versioning_automation automation pre-commit-check
"#;

const PRE_PUSH_HOOK_SCRIPT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
if ! command -v versioning_automation >/dev/null 2>&1; then
	echo "Error: command 'versioning_automation' not found." >&2
	exit 127
fi
exec versioning_automation automation pre-push-check
"#;

const POST_CHECKOUT_HOOK_SCRIPT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
IS_BRANCH_CHECKOUT="${3:-0}"
[[ "$IS_BRANCH_CHECKOUT" == "1" ]] || exit 0
[[ "${SKIP_POST_CHECKOUT_CONVENTION_WARN:-}" != "1" ]] || exit 0
if ! command -v versioning_automation >/dev/null 2>&1; then
	exit 0
fi
versioning_automation automation post-checkout-check || true
exit 0
"#;

const BRANCH_CREATION_CHECK_SCRIPT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
if ! command -v versioning_automation >/dev/null 2>&1; then
	echo "❌ versioning_automation not found" >&2
	echo "   Build/install it, then retry." >&2
	exit 127
fi
exec versioning_automation git branch-creation-check "$@"
"#;

fn run_gh_status(args: &[&str]) -> Result<(), String> {
    crate::gh_cli::status(args).map_err(|e| format!("Failed to run gh {}: {e}", args.join(" ")))
}

fn run_gh_output(args: &[&str]) -> Result<String, String> {
    crate::gh_cli::output_trim(args)
        .map_err(|e| format!("Failed to run gh {}: {e}", args.join(" ")))
}

fn branch_exists_local(branch_name: &str) -> bool {
    crate::git_cli::status_success(&[
        "show-ref",
        "--verify",
        "--quiet",
        &format!("refs/heads/{branch_name}"),
    ])
}

fn branch_exists_remote(remote: &str, branch_name: &str) -> bool {
    crate::git_cli::status_success(&["ls-remote", "--exit-code", "--heads", remote, branch_name])
}

fn parse_json_array(payload: &str, context: &str) -> Result<Vec<Json>, String> {
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
) -> Result<std::collections::HashMap<String, Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_object()
        .cloned()
        .ok_or_else(|| format!("Expected JSON object for {context}"))
}

fn parse_labels(content: &str, source_name: &str) -> Result<Vec<(String, String, String)>, String> {
    let parsed = parse_json_array(content, &format!("labels JSON '{source_name}'"))?;
    let mut labels = Vec::with_capacity(parsed.len());
    for label in parsed {
        let Some(object) = label.as_object() else {
            return Err(format!(
                "Invalid label entry in '{source_name}': expected object"
            ));
        };
        labels.push((
            object_string(object, "name"),
            object_string(object, "color"),
            object_string(object, "description"),
        ));
    }
    Ok(labels)
}

fn object_u64(object: &std::collections::HashMap<String, Json>, key: &str) -> u64 {
    object.get(key).and_then(Json::as_u64).unwrap_or(0)
}

fn object_string(object: &std::collections::HashMap<String, Json>, key: &str) -> String {
    object
        .get(key)
        .and_then(Json::as_str)
        .unwrap_or_default()
        .to_string()
}

fn object_string_or_default(
    object: &std::collections::HashMap<String, Json>,
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
