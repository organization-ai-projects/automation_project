use std::process::Command;

use crate::pr::commands::pr_issue_context_options::PrIssueContextOptions;
use crate::pr::contracts::github::issue_snapshot::IssueSnapshot;
use crate::pr::resolve_category::issue_category_from_title;

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
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("view")
        .arg(issue_number)
        .arg("--json")
        .arg("title,body,labels");

    let resolved_repo = repo
        .and_then(non_empty)
        .map(str::to_string)
        .or_else(|| {
            std::env::var("GH_REPO")
                .ok()
                .filter(|value| non_empty(value).is_some())
        })
        .or_else(resolve_repo_name_with_owner);

    if let Some(repo_name_with_owner) = resolved_repo {
        cmd.arg("-R").arg(repo_name_with_owner);
    }

    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn resolve_repo_name_with_owner() -> Option<String> {
    let output = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg("--json")
        .arg("nameWithOwner")
        .arg("-q")
        .arg(".nameWithOwner")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let repo = String::from_utf8_lossy(&output.stdout).trim().to_string();
    non_empty(&repo).map(str::to_string)
}

fn non_empty(value: &str) -> Option<&str> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}
