//! tools/versioning_automation/src/pr/issue_view.rs
use std::process::Command;

use crate::pr::commands::pr_issue_view_options::PrIssueViewOptions;

pub(crate) fn run_issue_view(opts: PrIssueViewOptions) -> i32 {
    let resolved_repo = opts
        .repo
        .and_then(|value| {
            if value.trim().is_empty() {
                None
            } else {
                Some(value)
            }
        })
        .or_else(|| {
            std::env::var("GH_REPO").ok().and_then(|value| {
                if value.trim().is_empty() {
                    None
                } else {
                    Some(value)
                }
            })
        });

    let mut command = Command::new("gh");
    command
        .arg("issue")
        .arg("view")
        .arg(&opts.issue_number)
        .arg("--json")
        .arg("title,body,labels");
    if let Some(repo_name) = resolved_repo {
        command.arg("-R").arg(repo_name);
    }

    match command.output() {
        Ok(output) if output.status.success() => {
            let json = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !json.is_empty() {
                println!("{json}");
            }
            0
        }
        _ => 0,
    }
}
