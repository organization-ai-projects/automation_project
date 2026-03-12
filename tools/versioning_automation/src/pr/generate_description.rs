use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::Command;

use regex::Regex;
use serde::Deserialize;

use crate::pr::breaking_detect::text_indicates_breaking;
use crate::pr::commands::pr_duplicate_actions_options::PrDuplicateActionsOptions;
use crate::pr::commands::pr_generate_description_options::PrGenerateDescriptionOptions;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::duplicate_actions::run_duplicate_actions;
use crate::pr::render::print_usage;
use crate::pr::scan::scan_directives;
use crate::repo_name::resolve_repo_name_optional;

const E_USAGE: i32 = 2;
const E_DEPENDENCY: i32 = 3;
const E_GIT: i32 = 4;
const E_NO_DATA: i32 = 5;

#[derive(Debug, Clone)]
struct GenerateOptions {
    help: bool,
    dry_run: bool,
    main_pr_number: Option<String>,
    create_pr: bool,
    allow_partial_create: bool,
    assume_yes: bool,
    base_ref: Option<String>,
    head_ref: Option<String>,
    duplicate_mode: Option<String>,
    auto_edit_pr_number: Option<String>,
    validation_only: bool,
    output_file: Option<String>,
}

#[derive(Debug, Clone)]
struct CommitInfo {
    short_hash: String,
    subject: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct MainPrRefSnapshot {
    #[serde(default, rename = "baseRefName")]
    base_ref_name: String,
    #[serde(default, rename = "headRefName")]
    head_ref_name: String,
}

#[derive(Debug, Deserialize)]
struct CompareResponse {
    #[serde(default)]
    commits: Vec<CompareCommit>,
}

#[derive(Debug, Deserialize)]
struct CompareCommit {
    #[serde(default)]
    sha: String,
    #[serde(default)]
    commit: CompareCommitDetail,
}

#[derive(Debug, Deserialize, Default)]
struct CompareCommitDetail {
    #[serde(default)]
    message: String,
}

pub(crate) fn run_generate_description(opts: PrGenerateDescriptionOptions) -> i32 {
    let parsed = match parse_generate_options(&opts.passthrough) {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{msg}");
            return E_USAGE;
        }
    };

    if parsed.help {
        print_usage();
        return 0;
    }

    run_native_dry_run(parsed)
}

fn run_native_dry_run(opts: GenerateOptions) -> i32 {
    let (base_ref, head_ref) = if opts.dry_run {
        let base_ref = opts.base_ref.unwrap_or_else(|| "dev".to_string());
        let head_ref = match opts.head_ref {
            Some(value) => value,
            None => match current_branch_name() {
                Ok(value) => value,
                Err(msg) => {
                    eprintln!("{msg}");
                    return E_GIT;
                }
            },
        };
        (base_ref, head_ref)
    } else {
        let main_pr_number = match opts.main_pr_number.as_deref() {
            Some(value) => value,
            None => {
                eprintln!("MAIN_PR_NUMBER is required.");
                return E_USAGE;
            }
        };
        let refs = match fetch_main_pr_refs(main_pr_number) {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{msg}");
                return E_DEPENDENCY;
            }
        };
        let base = if opts.base_ref.as_deref().unwrap_or("").trim().is_empty() {
            if refs.base_ref_name.trim().is_empty() {
                "dev".to_string()
            } else {
                refs.base_ref_name
            }
        } else {
            opts.base_ref.clone().unwrap_or_else(|| "dev".to_string())
        };
        let head = if opts.head_ref.as_deref().unwrap_or("").trim().is_empty() {
            if refs.head_ref_name.trim().is_empty() {
                "dev".to_string()
            } else {
                refs.head_ref_name
            }
        } else {
            opts.head_ref.clone().unwrap_or_else(|| "dev".to_string())
        };
        (base, head)
    };

    let range = format!("{base_ref}..{head_ref}");
    let mut commits = match git_log_commits(&range) {
        Ok(value) => value,
        Err(msg) => {
            if !opts.dry_run {
                eprintln!("Warning: {msg}");
            }
            Vec::new()
        }
    };

    if commits.is_empty() {
        if opts.dry_run {
            eprintln!(
                "Error: unable to determine commit messages for --dry-run compare {base_ref}...{head_ref}."
            );
            return E_NO_DATA;
        }
        commits = compare_api_commits(&base_ref, &head_ref).unwrap_or_default();
    }

    if commits.is_empty() {
        if opts.dry_run {
            eprintln!(
                "Error: unable to determine commit messages for --dry-run compare {base_ref}...{head_ref}."
            );
            return E_NO_DATA;
        }
        eprintln!("Error: unable to retrieve commit messages for {base_ref}..{head_ref}.");
        return E_DEPENDENCY;
    }

    let validation_gate = build_validation_gate(&commits);
    let duplicate_targets = collect_duplicate_targets(&commits);
    if let Some(mode) = opts.duplicate_mode.as_deref()
        && !opts.dry_run
    {
        let repo = match resolve_repo_name_optional(None) {
            Some(value) => value,
            None => {
                eprintln!("Warning: unable to resolve repository; duplicate mode skipped.");
                String::new()
            }
        };
        if !repo.is_empty() {
            let payload = duplicate_targets
                .iter()
                .map(|(dup, canonical)| format!("{dup}|{canonical}"))
                .collect::<Vec<String>>()
                .join("\n");
            let duplicate_status = run_duplicate_actions(PrDuplicateActionsOptions {
                text: payload,
                mode: mode.to_string(),
                repo,
                assume_yes: opts.assume_yes,
            });
            if duplicate_status != 0 {
                return duplicate_status;
            }
        }
    }

    let duplicate_message = opts.duplicate_mode.as_deref().and_then(|mode| {
        if opts.dry_run {
            Some(render_duplicate_mode_message(mode, &duplicate_targets))
        } else {
            None
        }
    });
    let body = if opts.validation_only {
        let pr_number = match opts.auto_edit_pr_number.as_deref() {
            Some(value) => value,
            None => {
                eprintln!("--validation-only requires --auto-edit/--refresh-pr.");
                return E_USAGE;
            }
        };
        let current_body = match gh_read_pr_body(pr_number) {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{msg}");
                return E_DEPENDENCY;
            }
        };
        replace_validation_gate(&current_body, &validation_gate)
    } else {
        build_full_body(&base_ref, &head_ref, &commits, &range, &validation_gate)
    };

    let exit_code = if let Some(pr_number) = opts.auto_edit_pr_number {
        match gh_edit_pr_body(&pr_number, &body) {
            Ok(()) => {
                println!("Updated PR body: #{pr_number}");
                0
            }
            Err(msg) => {
                eprintln!("{msg}");
                E_DEPENDENCY
            }
        }
    } else if opts.create_pr {
        if !opts.assume_yes {
            eprintln!("--yes is required for native --create-pr/--auto mode.");
            return E_USAGE;
        }

        let title = build_dynamic_pr_title(&base_ref, &head_ref, &commits);
        match gh_create_pr(&base_ref, &head_ref, &title, &body) {
            Ok(url_or_message) => {
                println!("PR created: {url_or_message}");
                0
            }
            Err(msg) => {
                if opts.allow_partial_create {
                    eprintln!("Warning: create-pr failed (partial allowed): {msg}");
                    0
                } else {
                    eprintln!("{msg}");
                    E_DEPENDENCY
                }
            }
        }
    } else if let Some(path) = opts.output_file {
        match fs::write(&path, &body) {
            Ok(()) => {
                println!("Generated file: {path}");
                0
            }
            Err(err) => {
                eprintln!("Failed to write output file '{path}': {err}");
                1
            }
        }
    } else {
        println!("{body}");
        0
    };

    if let Some(message) = duplicate_message {
        println!("{message}");
    }

    exit_code
}

fn build_full_body(
    base_ref: &str,
    head_ref: &str,
    commits: &[CommitInfo],
    range: &str,
    validation_gate: &str,
) -> String {
    let mut out = String::new();

    out.push_str("### Description\n\n");
    out.push_str(&format!(
        "This pull request merges the `{head_ref}` branch into `{base_ref}` and summarizes merged pull requests and resolved issues.\n\n"
    ));

    out.push_str(validation_gate);
    out.push_str("\n\n");

    out.push_str("### Issue Outcomes\n\n");
    out.push_str(&render_issue_outcomes(commits));
    out.push_str("\n\n");

    out.push_str("### Key Changes\n\n");
    out.push_str(&render_key_changes(commits));
    out.push_str("\n\n");

    out.push_str("#### Change Footprint\n\n");
    out.push_str(&render_change_footprint(range));

    out.trim_end().to_string()
}

fn render_issue_outcomes(commits: &[CommitInfo]) -> String {
    let mut closes = BTreeSet::new();
    let mut reopens = BTreeSet::new();

    let text = commits
        .iter()
        .map(|commit| format!("{}\n{}", commit.subject, commit.body))
        .collect::<Vec<String>>()
        .join("\n\n");

    for record in scan_directives(&text, true) {
        if record.first == "Closes" {
            closes.insert(record.second);
        } else if record.first == "Reopen" {
            reopens.insert(record.second);
        }
    }

    if closes.is_empty() && reopens.is_empty() {
        return "- No issues processed in this PR.".to_string();
    }

    let mut lines = Vec::new();
    if !closes.is_empty() {
        lines.push("#### Unknown".to_string());
        for key in &closes {
            lines.push(format!("- Closes {key}"));
        }
    }

    if !reopens.is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.push("#### Unknown".to_string());
        for key in &reopens {
            lines.push(format!("- Reopen {key}"));
        }
    }

    lines.join("\n")
}

fn render_key_changes(commits: &[CommitInfo]) -> String {
    let mut groups: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();

    for commit in commits {
        if commit.subject.trim().is_empty() {
            continue;
        }
        let category = classify_title(&commit.subject);
        groups
            .entry(category)
            .or_default()
            .push(format!("- {}", commit.subject.trim()));
    }

    let ordered = ["Synchronization", "Features", "Bug Fixes", "Refactoring"];
    let mut parts = Vec::new();

    for name in ordered {
        let Some(lines) = groups.get(name) else {
            continue;
        };
        if lines.is_empty() {
            continue;
        }
        parts.push(format!("#### {name}"));
        parts.push(String::new());
        for line in lines {
            parts.push(line.clone());
        }
        parts.push(String::new());
    }

    if parts.is_empty() {
        "- No significant items detected.".to_string()
    } else {
        parts.join("\n").trim_end().to_string()
    }
}

fn render_change_footprint(range: &str) -> String {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(range)
        .output();

    let Ok(output) = output else {
        return "- No changed files detected for this branch range.".to_string();
    };
    if !output.status.success() {
        return "- No changed files detected for this branch range.".to_string();
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    if files.is_empty() {
        return "- No changed files detected for this branch range.".to_string();
    }

    let mut docs = Vec::new();
    let mut shell = Vec::new();
    let mut crates = Vec::new();
    let mut workspace = Vec::new();
    let mut other = Vec::new();

    for file in files {
        if file.ends_with(".md")
            || file.starts_with("documentation/")
            || file.starts_with(".github/documentation/")
        {
            docs.push(file);
        } else if file.ends_with(".sh") || file.starts_with("scripts/") {
            shell.push(file);
        } else if file == "Cargo.toml"
            || file == "Cargo.lock"
            || file.starts_with(".cargo/")
            || file.ends_with("/Cargo.toml")
            || file.ends_with("/Cargo.lock")
            || file.starts_with("rust-toolchain")
        {
            workspace.push(file);
        } else if file.ends_with(".rs") || file.contains("/src/") {
            crates.push(file);
        } else {
            other.push(file);
        }
    }

    let mut lines = Vec::new();
    append_footprint_group(&mut lines, "Documentation", &docs, false);
    append_footprint_group(&mut lines, "Shell", &shell, false);
    append_footprint_group(&mut lines, "Crates", &crates, true);
    append_footprint_group(&mut lines, "Workspace", &workspace, false);
    append_footprint_group(&mut lines, "Other", &other, false);

    if lines.is_empty() {
        "- No changed files detected for this branch range.".to_string()
    } else {
        lines.join("\n")
    }
}

fn append_footprint_group(
    out: &mut Vec<String>,
    label: &str,
    files: &[String],
    aggregate_crates: bool,
) {
    if files.is_empty() {
        return;
    }

    out.push(format!("- {label} ({})", files.len()));

    if aggregate_crates && files.len() > 12 {
        let mut by_crate: BTreeMap<String, usize> = BTreeMap::new();
        for file in files {
            let key =
                infer_crate_from_path(file).unwrap_or_else(|| "(unresolved crate)".to_string());
            *by_crate.entry(key).or_insert(0) += 1;
        }

        for (name, count) in by_crate.into_iter().take(12) {
            out.push(format!("  - {name} ({count} files)"));
        }
        return;
    }

    for file in files.iter().take(12) {
        out.push(format!("  - {file}"));
    }
}

fn infer_crate_from_path(path: &str) -> Option<String> {
    let marker = "/src/";
    if let Some(index) = path.find(marker) {
        let prefix = &path[..index];
        if prefix.is_empty() {
            return None;
        }
        let segment = prefix.rsplit('/').next().unwrap_or(prefix);
        if !segment.is_empty() {
            return Some(segment.to_string());
        }
    }
    None
}

fn build_validation_gate(commits: &[CommitInfo]) -> String {
    let ci_status = "UNKNOWN ⚪";

    let mut breaking_commit_hashes = BTreeSet::new();
    let mut breaking_scopes = BTreeSet::new();

    let scope_re =
        Regex::new(r"^[\s]*[a-z][a-z0-9_-]*\(([a-z0-9_./,-]+)\)!?:").expect("valid regex");

    for commit in commits {
        let combined = format!("{}\n{}", commit.subject, commit.body);
        if !text_indicates_breaking(&combined) {
            continue;
        }

        breaking_commit_hashes.insert(commit.short_hash.clone());

        if let Some(caps) = scope_re.captures(commit.subject.trim()) {
            let scope = caps.get(1).map(|m| m.as_str().trim()).unwrap_or_default();
            if !scope.is_empty() {
                breaking_scopes.insert(scope.to_string());
            }
        }
    }

    let mut lines = vec![
        "### Validation Gate".to_string(),
        String::new(),
        format!("- CI: {ci_status}"),
    ];

    if breaking_commit_hashes.is_empty() {
        lines.push("- No breaking change".to_string());
    } else {
        lines.push("- Breaking change".to_string());
        lines.push("- Breaking scope:".to_string());

        if breaking_scopes.is_empty() {
            lines.push("  - crate(s): metadata-only (scope not inferable)".to_string());
        } else {
            let scopes = breaking_scopes
                .iter()
                .map(|v| format!("`{v}`"))
                .collect::<Vec<String>>()
                .join(", ");
            lines.push(format!("  - crate(s): {scopes}"));
        }

        let commits_value = breaking_commit_hashes
            .iter()
            .map(|v| format!("`{v}`"))
            .collect::<Vec<String>>()
            .join(", ");
        lines.push(format!("  - source commit(s): {commits_value}"));
    }

    lines.join("\n")
}

fn replace_validation_gate(body: &str, replacement: &str) -> String {
    replace_top_level_section(body, "### Validation Gate", replacement)
}

fn replace_top_level_section(body: &str, marker: &str, replacement: &str) -> String {
    let lines = body.lines().collect::<Vec<&str>>();
    let mut start = None;
    for (index, line) in lines.iter().enumerate() {
        if line.trim() == marker {
            start = Some(index);
            break;
        }
    }

    let Some(start) = start else {
        let mut base = body.trim_end().to_string();
        if !base.is_empty() {
            base.push_str("\n\n");
        }
        base.push_str(replacement);
        return base;
    };

    let mut end = lines.len();
    for (index, line) in lines.iter().enumerate().skip(start + 1) {
        if line.trim_start().starts_with("### ") {
            end = index;
            break;
        }
    }

    let mut out = Vec::new();
    out.extend(lines[..start].iter().map(|line| (*line).to_string()));
    out.push(replacement.to_string());
    if end < lines.len() {
        out.extend(lines[end..].iter().map(|line| (*line).to_string()));
    }

    out.join("\n").trim_end().to_string()
}

fn gh_read_pr_body(pr_number: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("--json")
        .arg("body")
        .arg("-q")
        .arg(".body // \"\"")
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end_matches('\n')
        .to_string())
}

fn gh_edit_pr_body(pr_number: &str, body: &str) -> Result<(), String> {
    let status = Command::new("gh")
        .arg("pr")
        .arg("edit")
        .arg(pr_number)
        .arg("--body")
        .arg(body)
        .status()
        .map_err(|err| format!("Failed to execute gh pr edit: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("gh pr edit failed for PR #{pr_number}"))
    }
}

fn git_log_commits(range: &str) -> Result<Vec<CommitInfo>, String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--format=%H%x1f%s%x1f%b%x1e")
        .arg(range)
        .output()
        .map_err(|err| format!("Error: failed to run git log for range {range}: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "Error: unable to read git history for range {range}."
        ));
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();

    for record in text.split('\x1e') {
        if record.trim().is_empty() {
            continue;
        }
        let mut parts = record.split('\x1f');
        let hash = parts.next().unwrap_or_default().trim();
        let subject = parts.next().unwrap_or_default().trim();
        let body = parts.next().unwrap_or_default().trim();
        if hash.is_empty() && subject.is_empty() && body.is_empty() {
            continue;
        }
        commits.push(CommitInfo {
            short_hash: hash.chars().take(7).collect::<String>(),
            subject: subject.to_string(),
            body: body.to_string(),
        });
    }

    Ok(commits)
}

fn current_branch_name() -> Result<String, String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .map_err(|err| format!("Error: failed to detect current branch: {err}"))?;

    if !output.status.success() {
        return Err("Error: unable to determine head branch in --dry-run mode.".to_string());
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        return Err("Error: unable to determine head branch in --dry-run mode.".to_string());
    }

    Ok(branch)
}

fn classify_title(title: &str) -> &'static str {
    let lower = title.to_lowercase();

    if lower.starts_with("merge ") || lower.contains("main into") || lower.contains("dev into") {
        return "Synchronization";
    }
    if lower.starts_with("fix") || lower.contains("bug") || lower.contains("hotfix") {
        return "Bug Fixes";
    }
    if lower.starts_with("refactor")
        || lower.starts_with("chore")
        || lower.contains("cleanup")
        || lower.contains("maintainability")
    {
        return "Refactoring";
    }
    "Features"
}

fn build_dynamic_pr_title(base_ref: &str, head_ref: &str, commits: &[CommitInfo]) -> String {
    let mut has_sync = false;
    let mut has_features = false;
    let mut has_bugs = false;
    let mut has_refactors = false;

    for commit in commits {
        match classify_title(&commit.subject) {
            "Synchronization" => has_sync = true,
            "Features" => has_features = true,
            "Bug Fixes" => has_bugs = true,
            "Refactoring" => has_refactors = true,
            _ => {}
        }
    }

    let mut categories = Vec::new();
    if has_sync {
        categories.push("Synchronization");
    }
    if has_features {
        categories.push("Features");
    }
    if has_bugs {
        categories.push("Bug Fixes");
    }
    if has_refactors {
        categories.push("Refactoring");
    }

    let summary = if categories.is_empty() {
        "Changes".to_string()
    } else if categories.len() == 1 {
        categories[0].to_string()
    } else if categories.len() == 2 {
        format!("{} and {}", categories[0], categories[1])
    } else {
        let mut text = categories[0].to_string();
        for item in categories
            .iter()
            .skip(1)
            .take(categories.len().saturating_sub(2))
        {
            text.push_str(", ");
            text.push_str(item);
        }
        text.push_str(", and ");
        text.push_str(categories.last().copied().unwrap_or("Changes"));
        text
    };

    format!("Merge {head_ref} into {base_ref}: {summary}")
}

fn collect_duplicate_targets(commits: &[CommitInfo]) -> BTreeMap<String, String> {
    let text = commits
        .iter()
        .map(|commit| format!("{}\n{}", commit.subject, commit.body))
        .collect::<Vec<String>>()
        .join("\n\n");

    let mut targets = BTreeMap::new();
    for record in scan_directives(&text, true) {
        if record.record_type != DirectiveRecordType::Duplicate {
            continue;
        }
        if !record.first.is_empty() && !record.second.is_empty() {
            targets.insert(record.first, record.second);
        }
    }

    targets
}

fn render_duplicate_mode_message(mode: &str, targets: &BTreeMap<String, String>) -> String {
    if targets.is_empty() {
        format!("Duplicate mode ({mode}): no duplicate declarations detected.")
    } else {
        format!("Duplicate mode ({mode}): dry-run simulation; no GitHub mutation applied.")
    }
}

fn gh_create_pr(base_ref: &str, head_ref: &str, title: &str, body: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("create")
        .arg("--base")
        .arg(base_ref)
        .arg("--head")
        .arg(head_ref)
        .arg("--title")
        .arg(title)
        .arg("--body")
        .arg(body)
        .arg("--label")
        .arg("pull-request")
        .output()
        .map_err(|err| format!("Failed to execute gh pr create: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        Ok("created".to_string())
    } else {
        Ok(text)
    }
}

fn parse_generate_options(args: &[String]) -> Result<GenerateOptions, String> {
    let mut help = false;
    let mut dry_run = false;
    let mut main_pr_number: Option<String> = None;
    let mut create_pr = false;
    let mut allow_partial_create = false;
    let mut assume_yes = false;
    let mut auto_mode = false;
    let mut mode_explicit = false;
    let mut base_ref: Option<String> = None;
    let mut head_ref: Option<String> = None;
    let mut duplicate_mode: Option<String> = None;
    let mut auto_edit_pr_number: Option<String> = None;
    let mut validation_only = false;
    let mut positionals: Vec<String> = Vec::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" => {
                dry_run = true;
                mode_explicit = true;
                i += 1;
            }
            "--base" => {
                base_ref = Some(take_value("--base", args, &mut i)?);
            }
            "--head" => {
                head_ref = Some(take_value("--head", args, &mut i)?);
            }
            "--create-pr" => {
                create_pr = true;
                mode_explicit = true;
                i += 1;
            }
            "--allow-partial-create" => {
                allow_partial_create = true;
                mode_explicit = true;
                i += 1;
            }
            "--yes" => {
                assume_yes = true;
                i += 1;
            }
            "--auto" => {
                auto_mode = true;
                mode_explicit = true;
                i += 1;
            }
            "--auto-edit" | "--refresh-pr" => {
                auto_edit_pr_number = Some(take_value(args[i].as_str(), args, &mut i)?);
            }
            "--validation-only" => {
                validation_only = true;
                i += 1;
            }
            "--duplicate-mode" => {
                duplicate_mode = Some(take_value("--duplicate-mode", args, &mut i)?);
            }
            // accepted/no-op flags for compatibility in migrated CI path
            "--debug" | "--keep-artifacts" => {
                i += 1;
            }
            "-h" | "--help" => {
                help = true;
                i += 1;
            }
            other if other.starts_with('-') => {
                return Err(format!("Unknown option for generate-description: {other}"));
            }
            _ => {
                positionals.push(args[i].clone());
                i += 1;
            }
        }
    }

    if help {
        return Ok(GenerateOptions {
            help: true,
            dry_run: false,
            main_pr_number: None,
            create_pr: false,
            allow_partial_create: false,
            assume_yes: false,
            base_ref: None,
            head_ref: None,
            duplicate_mode: None,
            auto_edit_pr_number: None,
            validation_only: false,
            output_file: None,
        });
    }

    if !mode_explicit && positionals.is_empty() {
        auto_mode = true;
    }
    if auto_mode {
        dry_run = true;
        create_pr = true;
        if !positionals.is_empty() {
            return Err("--auto does not accept a positional OUTPUT_FILE.".to_string());
        }
    }

    if create_pr && !dry_run {
        return Err("--create-pr is only supported with --dry-run.".to_string());
    }
    if allow_partial_create && !create_pr {
        return Err("--allow-partial-create requires --create-pr.".to_string());
    }
    if auto_edit_pr_number.is_some() && create_pr {
        return Err("--auto-edit cannot be combined with --create-pr/--auto.".to_string());
    }
    if validation_only && auto_edit_pr_number.is_none() {
        return Err("--validation-only requires --auto-edit/--refresh-pr.".to_string());
    }
    if let Some(mode) = duplicate_mode.as_deref()
        && mode != "safe"
        && mode != "auto-close"
    {
        return Err("--duplicate-mode must be 'safe' or 'auto-close'.".to_string());
    }

    let output_file = if dry_run && auto_edit_pr_number.is_none() {
        match positionals.len() {
            0 => None,
            1 => Some(positionals.remove(0)),
            _ => {
                return Err(
                    "Too many positional arguments for --dry-run. Only OUTPUT_FILE is allowed."
                        .to_string(),
                );
            }
        }
    } else {
        if dry_run && auto_edit_pr_number.is_some() && !positionals.is_empty() {
            return Err(
                "In --auto-edit dry-run mode, positional OUTPUT_FILE is not allowed.".to_string(),
            );
        }
        if auto_edit_pr_number.is_some() && positionals.len() > 1 {
            return Err(
                "In --auto-edit mode (MAIN_PR_NUMBER), positional OUTPUT_FILE is not allowed."
                    .to_string(),
            );
        }
        if auto_edit_pr_number.is_none() && positionals.len() > 2 {
            return Err(
                "Too many positional arguments. Expected usage: MAIN_PR_NUMBER [OUTPUT_FILE]."
                    .to_string(),
            );
        }
        if let Some(first) = positionals.first() {
            main_pr_number = Some(first.clone());
        }
        if main_pr_number.is_none() {
            return Err("MAIN_PR_NUMBER is required.".to_string());
        }
        if auto_edit_pr_number.is_none() {
            if positionals.len() >= 2 {
                Some(positionals[1].clone())
            } else {
                Some("pr_description.md".to_string())
            }
        } else {
            None
        }
    };

    Ok(GenerateOptions {
        help: false,
        dry_run,
        main_pr_number,
        create_pr,
        allow_partial_create,
        assume_yes,
        base_ref,
        head_ref,
        duplicate_mode,
        auto_edit_pr_number,
        validation_only,
        output_file,
    })
}

fn fetch_main_pr_refs(pr_number: &str) -> Result<MainPrRefSnapshot, String> {
    let mut cmd = Command::new("gh");
    cmd.arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("--json")
        .arg("baseRefName,headRefName");

    if let Some(repo) = resolve_repo_name_optional(None) {
        cmd.arg("-R").arg(repo);
    }

    let output = cmd
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    common_json::from_json_str::<MainPrRefSnapshot>(&json).map_err(|err| err.to_string())
}

fn compare_api_commits(base_ref: &str, head_ref: &str) -> Result<Vec<CommitInfo>, String> {
    let Some(repo) = resolve_repo_name_optional(None) else {
        return Err("Error: unable to determine repository.".to_string());
    };

    let output = Command::new("gh")
        .arg("api")
        .arg(format!("repos/{repo}/compare/{base_ref}...{head_ref}"))
        .output()
        .map_err(|err| format!("Failed to execute gh api compare: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    let parsed =
        common_json::from_json_str::<CompareResponse>(&json).map_err(|err| err.to_string())?;

    let mut commits = Vec::new();
    for entry in parsed.commits {
        let message = entry.commit.message.trim().to_string();
        if message.is_empty() {
            continue;
        }
        let mut lines = message.lines();
        let subject = lines.next().unwrap_or_default().trim().to_string();
        let body = lines.collect::<Vec<&str>>().join("\n").trim().to_string();
        commits.push(CommitInfo {
            short_hash: entry.sha.chars().take(7).collect::<String>(),
            subject,
            body,
        });
    }

    Ok(commits)
}

fn take_value(flag: &str, args: &[String], index: &mut usize) -> Result<String, String> {
    let value_index = *index + 1;
    if value_index >= args.len() {
        return Err(format!("{flag} requires a value"));
    }
    let value = args[value_index].clone();
    if value.starts_with('-') {
        return Err(format!("{flag} requires a value"));
    }
    *index += 2;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        CommitInfo, build_validation_gate, classify_title, parse_generate_options,
        render_duplicate_mode_message, replace_validation_gate,
    };
    use std::collections::BTreeMap;

    #[test]
    fn parse_dry_run_with_output_file() {
        let args = vec![
            "--dry-run".to_string(),
            "--base".to_string(),
            "dev".to_string(),
            "--head".to_string(),
            "feat/x".to_string(),
            "out.md".to_string(),
        ];
        let parsed = parse_generate_options(&args).expect("parse");
        assert!(parsed.dry_run);
        assert_eq!(parsed.base_ref.as_deref(), Some("dev"));
        assert_eq!(parsed.head_ref.as_deref(), Some("feat/x"));
        assert_eq!(parsed.output_file.as_deref(), Some("out.md"));
    }

    #[test]
    fn parse_validation_only_requires_auto_edit() {
        let args = vec!["--dry-run".to_string(), "--validation-only".to_string()];
        let err = parse_generate_options(&args).expect_err("must fail");
        assert!(err.contains("--validation-only requires --auto-edit/--refresh-pr"));
    }

    #[test]
    fn parse_auto_enables_dry_run_and_create_pr() {
        let args = vec![
            "--auto".to_string(),
            "--base".to_string(),
            "dev".to_string(),
            "--head".to_string(),
            "feat/x".to_string(),
            "--yes".to_string(),
        ];
        let parsed = parse_generate_options(&args).expect("parse");
        assert!(parsed.dry_run);
        assert!(parsed.create_pr);
        assert!(parsed.assume_yes);
    }

    #[test]
    fn parse_rejects_create_pr_without_dry_run() {
        let args = vec!["--create-pr".to_string()];
        let err = parse_generate_options(&args).expect_err("must fail");
        assert!(err.contains("--create-pr is only supported with --dry-run"));
    }

    #[test]
    fn parse_main_mode_requires_main_pr_number() {
        let err = parse_generate_options(&[]).expect("auto mode should parse");
        assert!(err.dry_run);
        assert!(err.create_pr);
    }

    #[test]
    fn parse_main_mode_accepts_main_and_output_file() {
        let args = vec!["42".to_string(), "out.md".to_string()];
        let parsed = parse_generate_options(&args).expect("parse");
        assert!(!parsed.dry_run);
        assert_eq!(parsed.main_pr_number.as_deref(), Some("42"));
        assert_eq!(parsed.output_file.as_deref(), Some("out.md"));
    }

    #[test]
    fn parse_without_mode_or_positionals_defaults_to_auto_even_with_base() {
        let args = vec!["--base".to_string(), "dev".to_string()];
        let parsed = parse_generate_options(&args).expect("should parse");
        assert!(parsed.dry_run);
        assert!(parsed.create_pr);
    }

    #[test]
    fn validation_gate_detects_breaking_commit() {
        let commits = vec![CommitInfo {
            short_hash: "abc1234".to_string(),
            subject: "feat(core)!: change wire format".to_string(),
            body: String::new(),
        }];
        let gate = build_validation_gate(&commits);
        assert!(gate.contains("- Breaking change"));
        assert!(gate.contains("`abc1234`"));
        assert!(gate.contains("`core`"));
    }

    #[test]
    fn replace_validation_gate_updates_existing_section() {
        let body = "### Description\n\nBody\n\n### Validation Gate\n\n- CI: PASS ✅\n- No breaking change\n\n### Key Changes\n\n- X";
        let replacement = "### Validation Gate\n\n- CI: UNKNOWN ⚪\n- No breaking change";
        let updated = replace_validation_gate(body, replacement);
        assert!(updated.contains("- CI: UNKNOWN ⚪"));
        assert!(!updated.contains("- CI: PASS ✅"));
        assert!(updated.contains("### Key Changes"));
    }

    #[test]
    fn classify_title_heuristics_cover_core_classes() {
        assert_eq!(classify_title("merge dev into main"), "Synchronization");
        assert_eq!(classify_title("fix(parser): handle nil"), "Bug Fixes");
        assert_eq!(
            classify_title("refactor(core): split modules"),
            "Refactoring"
        );
        assert_eq!(classify_title("feat(ui): add filter"), "Features");
    }

    #[test]
    fn duplicate_mode_message_reflects_presence_of_targets() {
        let empty = BTreeMap::new();
        let msg_empty = render_duplicate_mode_message("safe", &empty);
        assert!(msg_empty.contains("no duplicate declarations detected"));

        let mut one = BTreeMap::new();
        one.insert("#2".to_string(), "#1".to_string());
        let msg_one = render_duplicate_mode_message("auto-close", &one);
        assert!(msg_one.contains("dry-run simulation"));
    }
}
