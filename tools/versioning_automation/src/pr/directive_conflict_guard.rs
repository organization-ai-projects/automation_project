use std::collections::BTreeSet;
use std::process::Command;

use regex::Regex;

use crate::pr::closure_marker::apply_marker;
use crate::pr::commands::pr_directive_conflict_guard_options::PrDirectiveConflictGuardOptions;
use crate::pr::conflicts::build_conflict_report;

const BLOCK_START: &str = "<!-- directive-conflicts:start -->";
const BLOCK_END: &str = "<!-- directive-conflicts:end -->";

pub(crate) fn run_directive_conflict_guard(opts: PrDirectiveConflictGuardOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let original_body = match gh_output(
        "pr",
        &[
            "view",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--json",
            "body",
            "--jq",
            ".body // \"\"",
        ],
    ) {
        Ok(body) => body,
        Err(_) => {
            eprintln!("Error: unable to read PR #{}.", opts.pr_number);
            return 4;
        }
    };
    let mut updated_body = original_body.clone();

    let commit_messages = gh_output(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{}/commits", opts.pr_number),
            "--paginate",
            "--jq",
            ".[].commit.message",
        ],
    )
    .unwrap_or_default();
    let source_branch_count = detect_source_branch_count(&commit_messages);
    let directive_payload = format!("{commit_messages}\n{original_body}");

    let report = build_conflict_report(&directive_payload, source_branch_count);
    let resolved_count = report.resolved.len();
    let unresolved_count = report.unresolved.len();

    for entry in &report.resolved {
        if entry.decision != "close" {
            continue;
        }
        match apply_marker(&updated_body, "reopen|reopens", &entry.issue) {
            Ok(next) => updated_body = next,
            Err(err) => {
                eprintln!("{err}");
                return 2;
            }
        }
    }

    let marker = format!("<!-- directive-conflict-guard:{} -->", opts.pr_number);
    let conflict_block = build_conflict_block(&report);
    updated_body = upsert_conflict_block_in_body(&updated_body, conflict_block.as_deref());

    if updated_body != original_body {
        let status = gh_status(
            "pr",
            &[
                "edit",
                &opts.pr_number,
                "-R",
                &repo_name,
                "--body",
                &updated_body,
            ],
        );
        if status != 0 {
            return status;
        }
    }

    if unresolved_count > 0 {
        let comment_body = format!(
            "{marker}\n### Directive Conflict Guard\n\n❌ Unresolved Closes/Reopen conflicts detected. Add explicit directive decisions in PR body."
        );
        let status = upsert_pr_comment(&repo_name, &opts.pr_number, &marker, &comment_body);
        if status != 0 {
            return status;
        }
        eprintln!(
            "Unresolved directive conflicts detected for PR #{}.",
            opts.pr_number
        );
        return 8;
    }

    if resolved_count > 0 {
        let comment_body = format!(
            "{marker}\n### Directive Conflict Guard\n\n✅ Directive conflicts resolved via explicit decisions."
        );
        let status = upsert_pr_comment(&repo_name, &opts.pr_number, &marker, &comment_body);
        if status != 0 {
            return status;
        }
    }

    println!(
        "Directive conflict guard evaluated for PR #{}.",
        opts.pr_number
    );
    0
}

fn resolve_repo_name(explicit_repo: Option<String>) -> Result<String, String> {
    if let Some(repo) = explicit_repo.filter(|value| !value.trim().is_empty()) {
        return Ok(repo);
    }
    if let Ok(env_repo) = std::env::var("GH_REPO")
        && !env_repo.trim().is_empty()
    {
        return Ok(env_repo);
    }
    let resolved = gh_output(
        "repo",
        &["view", "--json", "nameWithOwner", "--jq", ".nameWithOwner"],
    )
    .unwrap_or_default();
    if resolved.trim().is_empty() {
        Err("Error: unable to determine repository.".to_string())
    } else {
        Ok(resolved)
    }
}

fn detect_source_branch_count(commit_messages: &str) -> u32 {
    let merge_re =
        Regex::new(r"(?m)^Merge pull request #[0-9]+ from [^/]+/(.+)$").expect("valid regex");
    let mut branches = BTreeSet::new();
    for caps in merge_re.captures_iter(commit_messages) {
        if let Some(branch) = caps.get(1) {
            let value = branch.as_str().trim();
            if !value.is_empty() {
                branches.insert(value.to_string());
            }
        }
    }
    if branches.is_empty() {
        1
    } else {
        branches.len() as u32
    }
}

fn build_conflict_block(
    report: &crate::pr::domain::conflicts::conflict_report::ConflictReport,
) -> Option<String> {
    if report.resolved.is_empty() && report.unresolved.is_empty() {
        return None;
    }

    let mut out = String::new();
    out.push_str(BLOCK_START);
    out.push('\n');
    out.push_str("### Issue Directive Decisions");
    out.push('\n');

    if !report.resolved.is_empty() {
        out.push('\n');
        out.push_str("Resolved decisions:");
        out.push('\n');
        let mut keys = report.resolved.iter().collect::<Vec<_>>();
        keys.sort_by_key(|entry| issue_number(&entry.issue));
        for entry in keys {
            out.push_str("- ");
            out.push_str(&entry.issue);
            out.push_str(" => ");
            out.push_str(&entry.decision);
            out.push_str(" (");
            out.push_str(&entry.origin);
            out.push_str(")\n");
        }
    }

    if !report.unresolved.is_empty() {
        out.push('\n');
        out.push_str("❌ Unresolved conflicts (merge blocked):");
        out.push('\n');
        let mut keys = report.unresolved.iter().collect::<Vec<_>>();
        keys.sort_by_key(|entry| issue_number(&entry.issue));
        for entry in keys {
            out.push_str("- ");
            out.push_str(&entry.issue);
            out.push_str(": ");
            out.push_str(&entry.reason);
            out.push('\n');
        }
        out.push('\n');
        out.push_str("Required decision format:\n");
        out.push_str("- `Directive Decision: #<issue> => close`\n");
        out.push_str("- `Directive Decision: #<issue> => reopen`\n");
    }

    out.push_str(BLOCK_END);
    Some(out)
}

fn upsert_conflict_block_in_body(body: &str, block: Option<&str>) -> String {
    let block_re = Regex::new(&format!(
        r"\n?{}\n.*?\n{}\n?",
        regex::escape(BLOCK_START),
        regex::escape(BLOCK_END)
    ))
    .expect("valid regex");
    let without_block = block_re.replace(body, "").to_string();

    match block {
        Some(content) => format!("{without_block}\n\n{content}\n"),
        None => without_block,
    }
}

fn upsert_pr_comment(repo_name: &str, pr_number: &str, marker: &str, body: &str) -> i32 {
    let list_path = format!("repos/{repo_name}/issues/{pr_number}/comments");
    let comment_id = gh_output(
        "api",
        &[
            &list_path,
            "--paginate",
            "--jq",
            &format!(
                "map(select((.body // \"\") | contains(\"{}\"))) | sort_by(.updated_at) | last | .id // empty",
                marker.replace('"', "\\\"")
            ),
        ],
    )
    .unwrap_or_default();

    if comment_id.trim().is_empty() {
        gh_status("api", &[&list_path, "-f", &format!("body={body}")])
    } else {
        gh_status(
            "api",
            &[
                "-X",
                "PATCH",
                &format!("repos/{repo_name}/issues/comments/{}", comment_id.trim()),
                "-f",
                &format!("body={body}"),
            ],
        )
    }
}

fn gh_status(cmd: &str, args: &[&str]) -> i32 {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute gh {}: {err}", cmd);
            1
        }
    }
}

fn gh_output(cmd: &str, args: &[&str]) -> Result<String, String> {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(text)
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
