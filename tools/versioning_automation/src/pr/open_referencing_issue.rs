use std::collections::BTreeSet;

use crate::pr::commands::pr_open_referencing_issue_options::PrOpenReferencingIssueOptions;
use crate::pr::gh_cli::gh_output_trim;
use crate::pr::text_payload::extract_effective_issue_ref_records;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_open_referencing_issue(opts: PrOpenReferencingIssueOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };
    let issue_key = format!("#{}", opts.issue_number);

    let pr_rows = gh_output_trim(
        "api",
        &[
            &format!("repos/{repo_name}/pulls?state=open&per_page=100"),
            "--paginate",
            "--jq",
            ".[]. | [.number, (.body // \"\")] | @tsv",
        ],
    )
    .unwrap_or_default();

    let mut matched = BTreeSet::new();
    for line in pr_rows.lines() {
        let mut parts = line.splitn(2, '\t');
        let Some(pr_number) = parts.next() else {
            continue;
        };
        let body = parts.next().unwrap_or_default();
        if pr_body_references_issue(body, &issue_key) {
            matched.insert(pr_number.to_string());
        }
    }

    for pr_number in matched {
        println!("{pr_number}");
    }
    0
}

fn pr_body_references_issue(body: &str, issue_key: &str) -> bool {
    for record in extract_effective_issue_ref_records(body) {
        if record.first == "Closes" && record.second == issue_key {
            return true;
        }
    }
    false
}
