use std::process::Command;

use crate::pr::commands::pr_issue_context_options::PrIssueContextOptions;
use crate::pr::contracts::github::issue_snapshot::IssueSnapshot;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::pr::resolve_category::issue_category_from_title;
use crate::repo_name::resolve_repo_name_optional;

pub(crate) fn run_issue_context(opts: PrIssueContextOptions) -> i32 {
    let payload = load_issue_context_payload(&opts);
    println!("{}\x1f{}\x1f{}", payload.0, payload.1, payload.2);
    0
}

fn load_issue_context_payload(opts: &PrIssueContextOptions) -> (String, String, String) {
    let Some(issue_json) = fetch_issue_json(&opts.issue_number, opts.repo.as_deref()) else {
        return (String::new(), "Unknown".to_string(), String::new());
    };

    let Ok(snapshot) = common_json::from_json_str::<IssueSnapshot>(&issue_json) else {
        return (String::new(), "Unknown".to_string(), String::new());
    };

    let labels_raw = snapshot
        .labels
        .iter()
        .map(|label| label.name.clone())
        .filter(|name| !name.trim().is_empty())
        .collect::<Vec<_>>()
        .join("||");

    let title_category = if snapshot.title.trim().is_empty() {
        "Unknown".to_string()
    } else {
        issue_category_from_title(&snapshot.title).to_string()
    };

    let reason = compute_non_compliance_reason(&snapshot.title, &snapshot.body, &labels_raw);

    (labels_raw, title_category, reason)
}

fn compute_non_compliance_reason(title: &str, body: &str, labels_raw: &str) -> String {
    let Ok(current_exe) = std::env::current_exe() else {
        return String::new();
    };

    let output = Command::new(current_exe)
        .arg("issue")
        .arg("non-compliance-reason")
        .arg("--title")
        .arg(title)
        .arg("--body")
        .arg(body)
        .arg("--labels-raw")
        .arg(labels_raw)
        .output();

    match output {
        Ok(result) if result.status.success() => {
            String::from_utf8_lossy(&result.stdout).trim().to_string()
        }
        _ => String::new(),
    }
}

fn fetch_issue_json(issue_number: &str, repo: Option<&str>) -> Option<String> {
    let resolved_repo = resolve_repo_name_optional(repo);
    let output = if let Some(repo_name_with_owner) = resolved_repo.as_deref() {
        gh_output_trim_end_newline(
            "issue",
            &[
                "view",
                issue_number,
                "--json",
                "title,body,labels",
                "-R",
                repo_name_with_owner,
            ],
        )
    } else {
        gh_output_trim_end_newline(
            "issue",
            &["view", issue_number, "--json", "title,body,labels"],
        )
    };
    output.ok()
}
