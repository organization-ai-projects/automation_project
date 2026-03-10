use std::collections::BTreeSet;
use std::process::Command;

use crate::pr::commands::pr_open_referencing_issue_options::PrOpenReferencingIssueOptions;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::scan::scan_directives;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_open_referencing_issue(opts: PrOpenReferencingIssueOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };
    let issue_key = format!("#{}", opts.issue_number);

    let pr_rows = gh_output(
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
    for record in scan_directives(body, false) {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }
        if record.first == "Closes" && record.second == issue_key {
            return true;
        }
    }
    false
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
