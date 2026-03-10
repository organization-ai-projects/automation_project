use std::collections::BTreeSet;
use std::process::Command;

use crate::pr::contracts::cli::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::contracts::directives::directive_record_type::DirectiveRecordType;
use crate::pr::contracts::github::pr_snapshot::PrSnapshot;
use crate::pr::scan::scan_directives;

const AUTO_BLOCK_START: &str = "<!-- auto-closes:start -->";
const AUTO_BLOCK_END: &str = "<!-- auto-closes:end -->";

pub(crate) fn run_auto_add_closes(opts: PrAutoAddClosesOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let pr_snapshot = match gh_pr_snapshot(&opts.pr_number, &repo_name) {
        Ok(snapshot) => snapshot,
        Err(_) => {
            eprintln!("Error: unable to read PR #{}.", opts.pr_number);
            return 3;
        }
    };
    let pr_state = pr_snapshot.state;
    let pr_base = pr_snapshot.base_ref_name;
    let pr_title = pr_snapshot.title;
    let pr_body = pr_snapshot.body;
    let pr_author = pr_snapshot
        .author
        .and_then(|author| author.login)
        .unwrap_or_default();

    if pr_state != "OPEN" {
        println!("PR #{} is not open; skipping.", opts.pr_number);
        return 0;
    }
    if pr_base != "dev" {
        println!("PR #{} does not target dev; skipping.", opts.pr_number);
        return 0;
    }
    if pr_author.is_empty() {
        println!(
            "PR #{}: author login unavailable; skipping.",
            opts.pr_number
        );
        return 0;
    }

    let pr_commits = gh_output(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{}/commits", opts.pr_number),
            "--paginate",
            "--jq",
            ".[].commit.message",
        ],
    )
    .unwrap_or_default();
    let payload_all = format!("{pr_title}\n{pr_body}\n{pr_commits}");

    let (part_of_refs, closing_refs) = collect_refs_from_payload(&payload_all);
    if part_of_refs.is_empty() {
        println!(
            "PR #{}: no Part of refs detected; nothing to enrich.",
            opts.pr_number
        );
        return 0;
    }

    let mut already_closing = BTreeSet::new();
    for issue_number in extract_issue_numbers(&closing_refs) {
        already_closing.insert(issue_number);
    }

    let mut closes_to_add = BTreeSet::new();
    for issue_number in extract_issue_numbers(&part_of_refs) {
        if already_closing.contains(&issue_number) {
            continue;
        }
        if should_close_issue_for_author(issue_number, &repo_name, &pr_author) {
            closes_to_add.insert(issue_number);
        }
    }

    if closes_to_add.is_empty() {
        println!(
            "PR #{}: no qualifying single-assignee issue found; nothing to enrich.",
            opts.pr_number
        );
        return 0;
    }

    let managed_block = build_managed_block(&closes_to_add);
    let body_without_block = collapse_blank_runs(&strip_managed_block(&pr_body));
    let new_body = if body_without_block.is_empty() {
        managed_block
    } else {
        format!("{body_without_block}\n\n{managed_block}")
    };

    if new_body == pr_body {
        println!("PR #{}: body already up-to-date.", opts.pr_number);
        return 0;
    }

    let mut edit = Command::new("gh");
    edit.arg("pr")
        .arg("edit")
        .arg(&opts.pr_number)
        .arg("-R")
        .arg(&repo_name)
        .arg("--body")
        .arg(&new_body);
    match edit.status() {
        Ok(status) if status.success() => {
            println!(
                "PR #{}: updated body with auto-managed Closes refs.",
                opts.pr_number
            );
            0
        }
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute gh pr edit: {err}");
            1
        }
    }
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

fn gh_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<PrSnapshot, String> {
    let json = gh_output(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "state,baseRefName,title,body,author",
        ],
    )?;
    common_json::from_json_str::<PrSnapshot>(&json).map_err(|err| err.to_string())
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

fn collect_refs_from_payload(payload: &str) -> (Vec<String>, Vec<String>) {
    let mut part_of_rows = BTreeSet::new();
    let mut closing_rows = BTreeSet::new();

    for record in scan_directives(payload, false) {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }
        if !record.second.starts_with('#') {
            continue;
        }

        if record.first == "Part of" {
            part_of_rows.insert(format!("Part of|{}", record.second));
        } else if record.first == "Closes" {
            closing_rows.insert(format!("Closes|{}", record.second));
        }
    }

    (
        part_of_rows.into_iter().collect(),
        closing_rows.into_iter().collect(),
    )
}

fn extract_issue_numbers(refs: &[String]) -> Vec<u32> {
    let mut issue_numbers = BTreeSet::new();
    for row in refs {
        let mut parts = row.split('|');
        let _action = parts.next();
        if let Some(issue_key) = parts.next()
            && let Some(number) = issue_key.strip_prefix('#')
            && let Ok(issue_number) = number.parse::<u32>()
        {
            issue_numbers.insert(issue_number);
        }
    }
    issue_numbers.into_iter().collect()
}

fn should_close_issue_for_author(issue_number: u32, repo_name: &str, pr_author: &str) -> bool {
    let assignees = gh_output(
        "issue",
        &[
            "view",
            &issue_number.to_string(),
            "-R",
            repo_name,
            "--json",
            "assignees",
            "--jq",
            ".assignees[].login",
        ],
    )
    .unwrap_or_default();

    let mut non_empty = assignees
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty());
    if let Some(first) = non_empty.next() {
        if non_empty.next().is_some() {
            return false;
        }
        first == pr_author
    } else {
        false
    }
}

fn build_managed_block(issue_numbers: &BTreeSet<u32>) -> String {
    let mut out = String::new();
    out.push_str(AUTO_BLOCK_START);
    out.push('\n');
    out.push_str("### Auto-managed Issue Closures");
    out.push('\n');
    for n in issue_numbers {
        out.push_str("Closes #");
        out.push_str(&n.to_string());
        out.push('\n');
    }
    out.push_str(AUTO_BLOCK_END);
    out
}

fn strip_managed_block(body: &str) -> String {
    let mut out_lines = Vec::new();
    let mut in_block = false;
    for line in body.lines() {
        if line == AUTO_BLOCK_START {
            in_block = true;
            continue;
        }
        if line == AUTO_BLOCK_END {
            in_block = false;
            continue;
        }
        if !in_block {
            out_lines.push(line);
        }
    }
    out_lines.join("\n")
}

fn collapse_blank_runs(text: &str) -> String {
    let mut current = text.to_string();
    while current.contains("\n\n\n") {
        current = current.replace("\n\n\n", "\n\n");
    }
    current
}
