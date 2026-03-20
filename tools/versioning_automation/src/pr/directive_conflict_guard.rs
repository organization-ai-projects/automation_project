use std::collections::BTreeSet;

use regex::Regex;

use crate::pr::closure_marker::apply_marker;
use crate::pr::commands::pr_directive_conflict_guard_options::PrDirectiveConflictGuardOptions;
use crate::pr::conflicts::build_conflict_report;
use crate::pr::gh_cli::{gh_output_trim, gh_status};
use crate::repo_name::resolve_repo_name;

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

    let original_body = match gh_output_trim(
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

    let commit_messages = gh_output_trim(
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
    let directive_payload = build_directive_payload(&original_body, &commit_messages);

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

pub(crate) fn build_directive_payload(body: &str, commit_messages: &str) -> String {
    format!("{body}\n{commit_messages}")
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
    let comment_id = gh_output_trim(
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

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
