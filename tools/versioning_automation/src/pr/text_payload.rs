//! tools/versioning_automation/src/pr/text_payload.rs
use std::collections::BTreeSet;

use crate::pr::commands::pr_text_payload_options::PrTextPayloadOptions;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::pr::state::build_state;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_text_payload(opts: PrTextPayloadOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    let payload = load_pr_text_payload(&opts.pr_number, &repo_name).unwrap_or_default();
    print!("{payload}");
    0
}

pub(crate) fn load_pr_text_payload(pr_number: &str, repo_name: &str) -> Result<String, String> {
    let title = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "title",
            "--jq",
            ".title // \"\"",
        ],
    )
    .map_err(|err| err.to_string())?;
    let body = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "body",
            "--jq",
            ".body // \"\"",
        ],
    )
    .map_err(|err| err.to_string())?;
    let commits = gh_output_trim_end_newline(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{pr_number}/commits"),
            "--paginate",
            "--jq",
            ".[].commit.message",
        ],
    )
    .map_err(|err| err.to_string())?;

    Ok(format!("{title}\n{body}\n{commits}"))
}

pub(crate) fn extract_effective_action_issue_numbers(
    payload: &str,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut closes = BTreeSet::new();
    let mut reopens = BTreeSet::new();

    for record in build_state(payload).action_records {
        let issue_number = record.second.trim_start_matches('#').to_string();
        if issue_number.is_empty() {
            continue;
        }
        match record.first.as_str() {
            "Closes" => {
                closes.insert(issue_number);
            }
            "Reopen" => {
                reopens.insert(issue_number);
            }
            _ => {}
        }
    }

    (closes, reopens)
}
