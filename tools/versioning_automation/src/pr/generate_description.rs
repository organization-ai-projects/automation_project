//! tools/versioning_automation/src/pr/generate_description.rs
use std::collections::BTreeMap;

use crate::gh_cli::{output_trim_cmd, status_cmd};
use crate::git_cli;
use crate::pr::group_by_category::GroupByCategory;
use crate::pr_remote_snapshot::PrRemoteSnapshot;
use crate::repo_name::resolve_repo_name_optional;

pub(crate) fn render_issue_outcome_entries(entries: &[(String, String)], action: &str) -> String {
    let records = entries
        .iter()
        .map(|entry| {
            GroupByCategory(
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
        .collect::<Vec<GroupByCategory>>();

    let rendered = GroupByCategory::render_issue_outcome_groups(&records);
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

pub(crate) fn render_directive_resolution_line(
    issue_key: &str,
    decision: &str,
    origin: &str,
) -> String {
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

pub(crate) fn render_change_footprint(range: &str) -> String {
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

pub(crate) fn replace_validation_gate(body: &str, replacement: &str) -> String {
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

pub(crate) fn gh_read_pr_body(pr_number: &str) -> Result<String, String> {
    let Some(repo) = resolve_repo_name_optional(None) else {
        return Err("Error: unable to determine repository.".to_string());
    };
    PrRemoteSnapshot::load_pr_remote_snapshot(pr_number, &repo).map(|snapshot| snapshot.body)
}

pub(crate) fn gh_edit_pr_body(pr_number: &str, body: &str) -> Result<(), String> {
    let Some(repo) = resolve_repo_name_optional(None) else {
        return Err("Error: unable to determine repository.".to_string());
    };
    let endpoint = format!("repos/{repo}/pulls/{pr_number}");
    status_cmd(
        "api",
        &[
            &endpoint,
            "--method",
            "PATCH",
            "-f",
            &format!("body={body}"),
        ],
    )
}

pub(crate) fn render_duplicate_mode_message(
    mode: &str,
    targets: &BTreeMap<String, String>,
) -> String {
    if targets.is_empty() {
        format!("Duplicate mode ({mode}): no duplicate declarations detected.")
    } else {
        format!("Duplicate mode ({mode}): dry-run simulation; no GitHub mutation applied.")
    }
}

pub(crate) fn gh_create_pr(
    base_ref: &str,
    head_ref: &str,
    title: &str,
    body: &str,
) -> Result<String, String> {
    let text = output_trim_cmd(
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
