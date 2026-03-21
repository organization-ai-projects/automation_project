use std::collections::BTreeMap;
use std::fs;

use crate::category_resolver::classify_title;
use crate::compare_snapshot::fetch_pr_refs;
use crate::pr::IssueOutcomesSnapshot;
use crate::pr::commands::pr_duplicate_actions_options::PrDuplicateActionsOptions;
use crate::pr::commands::pr_generate_description_options::PrGenerateDescriptionOptions;
use crate::pr::commit_info::CommitInfo;
use crate::pr::duplicate_actions::run_duplicate_actions;
use crate::pr::generate_options::GenerateOptions;
use crate::pr::gh_cli::{gh_output_trim, gh_output_trim_end_newline};
use crate::pr::group_by_category::CATEGORIES;
use crate::pr::render::print_usage;
use crate::pr_run_snapshot::load_pr_run_snapshot;
use crate::repo_name::resolve_repo_name_optional;
use crate::{git_cli, pr};

const E_USAGE: i32 = 2;
const E_DEPENDENCY: i32 = 3;
const E_GIT: i32 = 4;
const E_NO_DATA: i32 = 5;

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

    run_generate_flow(parsed)
}

fn run_generate_flow(opts: GenerateOptions) -> i32 {
    let (base_ref, head_ref) = if let Some(pr_number) = opts.auto_edit_pr_number.as_deref() {
        let refs = match fetch_pr_refs(pr_number) {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{msg}");
                return E_DEPENDENCY;
            }
        };
        let base_ref = if opts.base_ref.as_deref().unwrap_or("").trim().is_empty() {
            if refs.base_ref_name.trim().is_empty() {
                "dev".to_string()
            } else {
                refs.base_ref_name
            }
        } else {
            opts.base_ref.clone().unwrap_or_else(|| "dev".to_string())
        };
        let head_ref = if opts.head_ref.as_deref().unwrap_or("").trim().is_empty() {
            if refs.head_ref_name.trim().is_empty() {
                "dev".to_string()
            } else {
                refs.head_ref_name
            }
        } else {
            opts.head_ref.clone().unwrap_or_else(|| "dev".to_string())
        };
        (base_ref, head_ref)
    } else if let Some(main_pr_number) = opts.main_pr_number.as_deref() {
        let refs = match fetch_pr_refs(main_pr_number) {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{msg}");
                return E_DEPENDENCY;
            }
        };
        let base_ref = if opts.base_ref.as_deref().unwrap_or("").trim().is_empty() {
            if refs.base_ref_name.trim().is_empty() {
                "dev".to_string()
            } else {
                refs.base_ref_name
            }
        } else {
            opts.base_ref.clone().unwrap_or_else(|| "dev".to_string())
        };
        let head_ref = if opts.head_ref.as_deref().unwrap_or("").trim().is_empty() {
            if refs.head_ref_name.trim().is_empty() {
                "dev".to_string()
            } else {
                refs.head_ref_name
            }
        } else {
            opts.head_ref.clone().unwrap_or_else(|| "dev".to_string())
        };
        (base_ref, head_ref)
    } else {
        let base_ref = opts.base_ref.clone().unwrap_or_else(|| "dev".to_string());
        let head_ref = match opts.head_ref.clone() {
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
    };

    let run_snapshot = match load_pr_run_snapshot(&base_ref, &head_ref) {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{msg}");
            return E_DEPENDENCY;
        }
    };
    let range = format!(
        "{}..{}",
        run_snapshot.compare.base_ref, run_snapshot.compare.head_ref
    );
    let commits = run_snapshot.compare.commits;

    if commits.is_empty() {
        eprintln!("Error: unable to retrieve commit messages for {base_ref}...{head_ref}.");
        return E_NO_DATA;
    }

    let validation_gate = run_snapshot.validation_gate;
    let duplicate_targets = run_snapshot.duplicate_targets;
    let issue_outcomes = run_snapshot.issue_outcomes;
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
        build_full_body(
            &base_ref,
            &head_ref,
            &commits,
            &range,
            &validation_gate,
            &issue_outcomes,
        )
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

pub(crate) fn build_full_body(
    base_ref: &str,
    head_ref: &str,
    commits: &[CommitInfo],
    range: &str,
    validation_gate: &str,
    issue_outcomes: &IssueOutcomesSnapshot,
) -> String {
    let mut out = String::new();

    out.push_str("### Description\n\n");
    out.push_str(&format!(
        "This pull request merges the `{head_ref}` branch into `{base_ref}` and summarizes merged pull requests and resolved issues.\n\n"
    ));

    out.push_str(validation_gate);
    out.push_str("\n\n");

    out.push_str("### Issue Outcomes\n\n");
    out.push_str(&render_issue_outcomes(issue_outcomes));
    out.push_str("\n\n");

    out.push_str("### Key Changes\n\n");
    out.push_str(&render_key_changes(commits));
    out.push_str("\n\n");

    out.push_str("#### Change Footprint\n\n");
    out.push_str(&render_change_footprint(range));

    out.trim_end().to_string()
}

fn render_issue_outcomes(snapshot: &IssueOutcomesSnapshot) -> String {
    if snapshot.is_empty() {
        return "- No issues processed in this PR.".to_string();
    }

    let close_rendered = render_issue_outcome_entries(&snapshot.close_only, "Closes");
    let reopen_rendered = render_issue_outcome_entries(&snapshot.reopen_only, "Reopen");
    let directive_resolution_records = snapshot
        .resolved_conflicts
        .iter()
        .map(|entry| {
            (
                entry
                    .0
                    .trim_start_matches('#')
                    .parse::<u32>()
                    .unwrap_or(u32::MAX),
                entry.1.clone(),
                vec![render_directive_resolution_line(
                    &entry.0, &entry.2, &entry.3,
                )],
                0usize,
            )
        })
        .collect::<Vec<_>>();
    let unresolved_conflict_records = snapshot
        .unresolved_conflicts
        .iter()
        .map(|entry| {
            (
                entry
                    .0
                    .trim_start_matches('#')
                    .parse::<u32>()
                    .unwrap_or(u32::MAX),
                entry.1.clone(),
                vec![entry.0.clone(), entry.2.clone()],
                0usize,
            )
        })
        .collect::<Vec<_>>();
    let directive_rendered =
        render_issue_outcome_groups_with_mode(&directive_resolution_records, "directive")
            .trim()
            .to_string();
    let unresolved_rendered =
        render_issue_outcome_groups_with_mode(&unresolved_conflict_records, "conflict")
            .trim()
            .to_string();

    let mut out = String::new();
    out.push_str("#### Category 1: Issues Without Conflicts\n\n");
    out.push_str("##### Closes/Fixes\n\n");
    if close_rendered.trim().is_empty() {
        out.push_str(
            "- No resolved issues detected via GitHub references or PR body keywords.\n\n",
        );
    } else {
        out.push_str(close_rendered.trim());
        out.push_str("\n\n");
    }

    out.push_str("##### Reopened\n\n");
    if reopen_rendered.trim().is_empty() {
        out.push_str("- No reopened issues detected.\n\n");
    } else {
        out.push_str(reopen_rendered.trim());
        out.push_str("\n\n");
    }

    out.push_str("#### Category 2: Issues With Conflicts\n\n");
    out.push_str("##### Auto-resolved\n\n");
    if directive_rendered.trim().is_empty() {
        out.push_str("- No auto-resolved directive conflicts.\n\n");
    } else {
        out.push_str(directive_rendered.trim());
        out.push_str("\n\n");
    }

    out.push_str("##### Not resolved\n\n");
    if unresolved_rendered.trim().is_empty() {
        out.push_str("- No unresolved directive conflicts.");
    } else {
        out.push_str(unresolved_rendered.trim());
    }

    out
}

fn render_issue_outcome_entries(entries: &[(String, String)], action: &str) -> String {
    let records = entries
        .iter()
        .map(|entry| {
            (
                entry
                    .0
                    .trim_start_matches('#')
                    .parse::<u32>()
                    .unwrap_or(u32::MAX),
                entry.1.clone(),
                vec![action.to_string(), entry.0.clone()],
                0usize,
            )
        })
        .collect::<Vec<_>>();

    let rendered = render_issue_outcome_groups(&records);
    if rendered.trim().is_empty() {
        entries
            .iter()
            .map(|entry| format!("- {action} {}", entry.0))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        rendered
    }
}

fn render_issue_outcome_groups(records: &[pr::group_by_category::GroupByCategory]) -> String {
    render_issue_outcome_groups_with_mode(records, "resolved")
}

pub(crate) fn render_issue_outcome_groups_with_mode(
    records: &[pr::group_by_category::GroupByCategory],
    mode: &str,
) -> String {
    let mut out = String::new();
    for category in CATEGORIES {
        let matching = records
            .iter()
            .filter(|record| record.1 == category)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            continue;
        }

        out.push_str("#### ");
        out.push_str(category);
        out.push('\n');

        for record in matching {
            let action = record.2.first().cloned().unwrap_or_default();
            let issue_key = record.2.get(1).cloned().unwrap_or_default();
            let line = if mode == "conflict" {
                format!("- {action} - {issue_key}")
            } else if mode == "directive" {
                format!("- {action}")
            } else {
                format!("- {action} {issue_key}")
            };
            out.push_str(&line);
            out.push('\n');
        }
        out.push('\n');
    }
    out
}

fn render_directive_resolution_line(issue_key: &str, decision: &str, origin: &str) -> String {
    let resolved_action = if decision == "close" {
        "close"
    } else {
        "reopen"
    };
    if origin == "explicit" || origin == "inferred from latest directive" {
        let prefix = if decision == "close" {
            "Closes"
        } else {
            "Reopen"
        };
        format!("{prefix} {issue_key} - Resolved via directive decision => {resolved_action}.")
    } else {
        let prefix = if decision == "close" {
            "Closes"
        } else {
            "Reopen"
        };
        format!(
            "{prefix} {issue_key} - resolved Closes/Reopen conflict; winner: {resolved_action}; origin: {origin}."
        )
    }
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
    let Ok(output) = git_cli::output_preserve(&["diff", "--name-only", range]) else {
        return "- No changed files detected for this branch range.".to_string();
    };

    let files = output
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
    gh_output_trim_end_newline(
        "pr",
        &["view", pr_number, "--json", "body", "-q", ".body // \"\""],
    )
}

fn gh_edit_pr_body(pr_number: &str, body: &str) -> Result<(), String> {
    let Some(repo) = resolve_repo_name_optional(None) else {
        return Err("Error: unable to determine repository.".to_string());
    };
    let endpoint = format!("repos/{repo}/pulls/{pr_number}");
    gh_output_trim(
        "api",
        &[
            &endpoint,
            "--method",
            "PATCH",
            "-f",
            &format!("body={body}"),
        ],
    )
    .map(|_| ())
}

fn current_branch_name() -> Result<String, String> {
    let branch = git_cli::output_trim(&["rev-parse", "--abbrev-ref", "HEAD"])
        .map_err(|err| format!("Error: failed to detect current branch: {err}"))?;
    if branch.is_empty() {
        return Err("Error: unable to determine head branch in --dry-run mode.".to_string());
    }

    Ok(branch)
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

fn render_duplicate_mode_message(mode: &str, targets: &BTreeMap<String, String>) -> String {
    if targets.is_empty() {
        format!("Duplicate mode ({mode}): no duplicate declarations detected.")
    } else {
        format!("Duplicate mode ({mode}): dry-run simulation; no GitHub mutation applied.")
    }
}

fn gh_create_pr(base_ref: &str, head_ref: &str, title: &str, body: &str) -> Result<String, String> {
    let text = gh_output_trim(
        "pr",
        &[
            "create",
            "--base",
            base_ref,
            "--head",
            head_ref,
            "--title",
            title,
            "--body",
            body,
            "--label",
            "pull-request",
        ],
    )?;
    if text.is_empty() {
        Ok("created".to_string())
    } else {
        Ok(text)
    }
}

pub(crate) fn parse_generate_options(args: &[String]) -> Result<GenerateOptions, String> {
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
                mode_explicit = true;
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
        create_pr = true;
        if !positionals.is_empty() {
            return Err("--auto does not accept a positional OUTPUT_FILE.".to_string());
        }
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
        if auto_edit_pr_number.is_none() && !create_pr && positionals.len() > 2 {
            return Err(
                "Too many positional arguments. Expected usage: MAIN_PR_NUMBER [OUTPUT_FILE]."
                    .to_string(),
            );
        }
        if auto_edit_pr_number.is_none()
            && !create_pr
            && let Some(first) = positionals.first()
        {
            main_pr_number = Some(first.clone());
        }
        if auto_edit_pr_number.is_none() && !create_pr && main_pr_number.is_none() {
            return Err("MAIN_PR_NUMBER is required.".to_string());
        }
        if auto_edit_pr_number.is_none() && !create_pr {
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
