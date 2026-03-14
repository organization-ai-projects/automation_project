//! tools/versioning_automation/src/automation/execute.rs
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use common_json::Json;

use crate::automation::commands::{
    AuditSecurityOptions, AutomationAction, BuildAccountsUiOptions, BuildAndCheckUiBundlesOptions,
    BuildUiBundlesOptions, ChangedCratesOptions, CheckDependenciesOptions,
    CheckMergeConflictsOptions, CheckPriorityIssuesOptions, CiWatchPrOptions,
    CleanArtifactsOptions, LabelsSyncOptions, PreAddReviewOptions, SyncMainDevCiOptions,
    TestCoverageOptions,
};
use crate::automation::parse::parse;
use crate::automation::render::print_usage;

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
        AutomationAction::AuditSecurity(opts) => run_audit_security(opts),
        AutomationAction::BuildAccountsUi(opts) => run_build_accounts_ui(opts),
        AutomationAction::BuildUiBundles(opts) => run_build_ui_bundles(opts),
        AutomationAction::BuildAndCheckUiBundles(opts) => run_build_and_check_ui_bundles(opts),
        AutomationAction::PreAddReview(opts) => run_pre_add_review(opts),
        AutomationAction::TestCoverage(opts) => run_test_coverage(opts),
        AutomationAction::ChangedCrates(opts) => run_changed_crates(opts),
        AutomationAction::CheckMergeConflicts(opts) => run_check_merge_conflicts(opts),
        AutomationAction::CheckDependencies(opts) => run_check_dependencies(opts),
        AutomationAction::CleanArtifacts(opts) => run_clean_artifacts(opts),
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
