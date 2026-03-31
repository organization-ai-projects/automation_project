//! tools/versioning_automation/src/automation/execute.rs
use std::collections::{self, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use crate::lazy_regex::{COMMIT_MESSAGE_FORMAT_REGEX, ISSUE_PREFIX_REGEX, SCOPE_EXTRACTION_REGEX};
use crate::pr::extract_effective_issue_ref_records;
use common_json::Json;

use crate::parent_field::extract_parent_field;
use crate::repo_name::resolve_repo_name_optional;
use crate::{gh_cli, git_cli};

pub(crate) fn to_exit_code(result: Result<(), String>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

pub(crate) fn first_non_comment_subject_line(message: &str) -> Option<String> {
    message
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToString::to_string)
}

pub(crate) fn parse_subject_max_len() -> Result<Option<usize>, String> {
    let raw = env::var("COMMIT_MSG_SUBJECT_MAX_LEN").unwrap_or_default();
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

pub(crate) fn has_non_comment_content(message: &str) -> bool {
    message
        .lines()
        .map(str::trim)
        .any(|line| !line.is_empty() && !line.starts_with('#'))
}

pub(crate) fn collect_format_categories(staged_files: &[String]) -> Vec<String> {
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

pub(crate) fn detect_required_scopes(staged_files: &[String]) -> Result<Vec<String>, String> {
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

pub(crate) fn extract_scopes_from_commit_subject(subject: &str) -> Vec<String> {
    let re = match SCOPE_EXTRACTION_REGEX.as_ref() {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Regex error: {e}");
            return Vec::new();
        }
    };
    let captures = match re.captures(subject) {
        Some(captures) => captures,
        None => return Vec::new(),
    };

    let raw: &str = match captures.get(1) {
        Some(raw) => raw.as_str(),
        None => return Vec::new(),
    };

    let scopes: Vec<String> = raw
        .split(',')
        .map(|scope| scope.trim().to_string())
        .filter(|scope| !scope.is_empty())
        .collect();

    scopes
}

pub(crate) fn detect_commit_type_from_context(
    staged_files: &[String],
) -> (String, Option<&'static str>) {
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

pub(crate) fn is_docs_only_change(staged_files: &[String]) -> bool {
    !staged_files.is_empty()
        && staged_files.iter().all(|file| {
            file.ends_with(".md") || file.starts_with("documentation/") || file.starts_with("docs/")
        })
}

pub(crate) fn is_tests_only_change(staged_files: &[String]) -> bool {
    !staged_files.is_empty()
        && staged_files.iter().all(|file| {
            file.contains("/tests/")
                || file.starts_with("tests/")
                || file.ends_with("_test.rs")
                || file.ends_with(".snap")
        })
}

pub(crate) fn derive_description(branch: &str, staged_files: &[String]) -> String {
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
    if let Some(re) = ISSUE_PREFIX_REGEX.as_ref().ok()
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

pub(crate) fn run_audit_security() -> Result<(), String> {
    ensure_git_repo()?;
    if !command_available("cargo-audit") {
        return Err("cargo-audit not found. Install with: cargo install cargo-audit".to_string());
    }
    let _ = run_command_status("cargo", &["audit", "fetch"], true);
    run_command_status("cargo", &["audit"], false)
}

pub(crate) fn run_post_checkout_check() -> Result<(), String> {
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

pub(crate) fn run_test_coverage() -> Result<(), String> {
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

    let formats_raw = env::var("COVERAGE_FORMATS").unwrap_or_else(|_| "html".to_string());
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
            || env::var("ALLOW_PART_OF_ONLY_PUSH").unwrap_or_default() == "1"
        {
            println!("Assignment policy check skipped (repo unresolved).");
            return Ok(());
        }
        return Err("Cannot resolve GitHub repository for Part-of assignment policy.".to_string());
    };
    let current_login = gh_cli::output_trim(&["api", "user", "--jq", ".login"]).unwrap_or_default();
    if current_login.trim().is_empty() {
        if remote_policy_warn_only()
            || env::var("ALLOW_PART_OF_ONLY_PUSH").unwrap_or_default() == "1"
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
        let assignees = gh_cli::output_trim(&[
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
    if !violations.is_empty() && env::var("ALLOW_PART_OF_ONLY_PUSH").unwrap_or_default() != "1" {
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
    let hooks_policy = env::var("HOOKS_REMOTE_POLICY").unwrap_or_default();
    let allow_offline = env::var("ALLOW_OFFLINE_REMOTE_CHECKS").unwrap_or_default();
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
    let body = gh_cli::output_trim(&[
        "issue",
        "view",
        issue_number,
        "-R",
        repo,
        "--json",
        "body",
        "--jq",
        ".body // \"\"",
    ])
    .map_err(|e| {
        format!(
            "Failed to run gh issue view {} -R {} --json body --jq .body // \"\": {e}",
            issue_number, repo
        )
    })?;
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

pub(crate) fn extract_subissue_refs_for_parent(
    repo_owner: &str,
    repo_short_name: &str,
    parent_number: &str,
) -> Result<Vec<String>, String> {
    let output = gh_cli::output_trim(&[
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
    ])
    .map_err(|e| format!("Failed to run gh api graphql for subissues of #{}: {e}", parent_number))?;
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
    git_rev_parse(&["--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .unwrap_or_else(|_| "origin/dev".to_string())
}

pub(crate) fn ensure_git_repo() -> Result<(), String> {
    if git_rev_parse(&["--is-inside-work-tree"]).is_ok() {
        Ok(())
    } else {
        Err("Not a git repository".to_string())
    }
}

pub(crate) fn current_branch() -> Result<String, String> {
    let branch = git_branch(&["--show-current"])?;
    if branch.is_empty() {
        return Err("Not on a branch (detached HEAD).".to_string());
    }
    Ok(branch)
}

pub(crate) fn current_branch_name() -> Result<String, String> {
    git_rev_parse(&["--abbrev-ref", "HEAD"])
        .map_err(|e| format!("Failed to get current branch name: {e}"))
}

pub(crate) fn get_merged_branches(base_branch: &str) -> Result<Vec<String>, String> {
    let output = git_branch(&["--merged", base_branch])?;
    let branches = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect();
    Ok(branches)
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

pub(crate) fn repo_root() -> Result<PathBuf, String> {
    let root = git_rev_parse(&["--show-toplevel"])?;
    if root.trim().is_empty() {
        return Err("Unable to resolve git repository root.".to_string());
    }
    Ok(PathBuf::from(root))
}

pub(crate) fn find_crate_dir_for_file(repo_root: &Path, file: &str) -> Option<String> {
    let mut cursor = repo_root.join(file);
    if cursor.extension().is_some() || !cursor.is_dir() {
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

pub(crate) fn branch_exists_remote(remote: &str, branch_name: &str) -> bool {
    git_cli::status_success(&["ls-remote", "--exit-code", "--heads", remote, branch_name])
}

pub(crate) fn run_git_output(args: &[&str]) -> Result<String, String> {
    git_cli::output_trim(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

pub(crate) fn run_git_output_preserve(args: &[&str]) -> Result<String, String> {
    git_cli::output_preserve(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

pub(crate) fn git_hooks_dir(root: &Path) -> Result<PathBuf, String> {
    let output = git_rev_parse(&["--git-path", "hooks"])?;
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

pub(crate) fn branch_exists_local(branch_name: &str) -> bool {
    git_cli::status_success(&[
        "show-ref",
        "--verify",
        "--quiet",
        &format!("refs/heads/{branch_name}"),
    ])
}

pub(crate) fn parse_json_array(payload: &str, context: &str) -> Result<Vec<Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_array()
        .cloned()
        .ok_or_else(|| format!("Expected JSON array for {context}"))
}

pub(crate) fn parse_json_object(
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

pub(crate) fn object_string_or_default(
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

pub(crate) fn run_git_passthrough(args: &[&str]) -> Result<(), String> {
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

pub(crate) fn run_git_output_allow_failure(args: &[&str]) -> Option<String> {
    git_cli::output_trim(args).ok()
}

pub(crate) fn git_fetch_prune(remote: &str) -> Result<(), String> {
    run_git(&["fetch", remote, "--prune"])
}

pub(crate) fn is_protected_branch(branch_name: &str) -> bool {
    matches!(branch_name, "main" | "dev")
}

pub(crate) fn require_non_protected_branch(branch_name: &str) -> Result<(), String> {
    if is_protected_branch(branch_name) {
        return Err(format!("Cannot operate on protected branch: {branch_name}"));
    }
    Ok(())
}

pub(crate) fn validate_branch_name(branch_name: &str) -> Result<(), String> {
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

pub(crate) fn validate_branch_type(branch_type: &str) -> Result<(), String> {
    match branch_type {
        "feature" | "feat" | "fixture" | "fix" | "chore" | "refactor" | "doc" | "docs" | "test"
        | "tests" => Ok(()),
        _ => Err(format!(
            "Invalid type '{branch_type}'. Must be one of: feature, feat, fixture, fix, chore, refactor, doc, docs, test, tests"
        )),
    }
}

pub(crate) fn sanitize_description(description: &str) -> String {
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

pub(crate) fn require_clean_tree() -> Result<(), String> {
    let unstaged_clean = git_cli::status_success(&["diff", "--quiet"]);
    let staged_clean = git_cli::status_success(&["diff", "--cached", "--quiet"]);

    if unstaged_clean && staged_clean {
        Ok(())
    } else {
        Err("Working tree is dirty. Commit/stash your changes first.".to_string())
    }
}

//if success returns true, if failed returns false.
pub(crate) fn has_upstream() -> bool {
    git_rev_parse(&["--abbrev-ref", "--symbolic-full-name", "@{u}"]).is_ok()
}

pub(crate) fn validate_commit_message(message: &str) -> Result<(), String> {
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

pub(crate) fn list_gone_branches() -> Result<Vec<String>, String> {
    let output = git_branch(&["-vv"])?;
    let mut branches = collections::BTreeSet::new();
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
            branches.insert(branch.to_string());
        }
    }
    Ok(branches.into_iter().collect())
}

pub(crate) fn last_deleted_branch_file() -> Option<PathBuf> {
    let git_dir = git_rev_parse(&["--git-dir"]).ok()?;
    Some(PathBuf::from(git_dir).join("last_deleted_branch"))
}

pub(crate) fn save_last_deleted_branch(branch_name: &str) -> Result<(), String> {
    let Some(path) = last_deleted_branch_file() else {
        return Ok(());
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("Cannot create state dir: {err}"))?;
    }

    fs::write(path, branch_name).map_err(|err| format!("Cannot write state file: {err}"))
}

pub(crate) fn load_last_deleted_branch() -> Option<String> {
    let path = last_deleted_branch_file()?;
    let content = fs::read_to_string(path).ok()?;
    let value = content.trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}

fn git_rev_parse(args: &[&str]) -> Result<String, String> {
    let mut cmd = vec!["rev-parse"];
    cmd.extend_from_slice(args);
    git_cli::output_trim(&cmd)
        .map_err(|e| format!("Failed to run git rev-parse {}: {e}", args.join(" ")))
}

fn git_branch(args: &[&str]) -> Result<String, String> {
    let mut cmd = vec!["branch"];
    cmd.extend_from_slice(args);
    git_cli::output_trim(&cmd)
        .map_err(|e| format!("Failed to run git branch {}: {e}", args.join(" ")))
}

pub(crate) fn resolve_branch_sha(branch_ref: &str) -> Result<String, String> {
    git_rev_parse(&[branch_ref])
}
