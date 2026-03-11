use serde::Deserialize;
use std::process::Command;

use crate::pr::commands::pr_field_name::PrFieldName;
use crate::pr::commands::pr_field_options::PrFieldOptions;
use crate::repo_name::resolve_repo_name;

#[derive(Debug, Deserialize)]
struct GhPrFieldSnapshot {
    #[serde(default)]
    state: String,
    #[serde(default, rename = "baseRefName")]
    base_ref_name: String,
    #[serde(default, rename = "headRefName")]
    head_ref_name: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    author: Option<GhPrFieldAuthor>,
}

#[derive(Debug, Deserialize)]
struct GhPrFieldAuthor {
    #[serde(default)]
    login: String,
}

pub(crate) fn run_field(opts: PrFieldOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    match opts.name {
        PrFieldName::CommitMessages => {
            let out = fetch_commit_messages(&opts.pr_number, &repo_name).unwrap_or_default();
            print!("{out}");
            0
        }
        _ => {
            let snapshot =
                fetch_pr_snapshot(&opts.pr_number, &repo_name).unwrap_or(GhPrFieldSnapshot {
                    state: String::new(),
                    base_ref_name: String::new(),
                    head_ref_name: String::new(),
                    title: String::new(),
                    body: String::new(),
                    author: None,
                });
            let value = match opts.name {
                PrFieldName::State => snapshot.state,
                PrFieldName::BaseRefName => snapshot.base_ref_name,
                PrFieldName::HeadRefName => snapshot.head_ref_name,
                PrFieldName::Title => snapshot.title,
                PrFieldName::Body => snapshot.body,
                PrFieldName::AuthorLogin => snapshot
                    .author
                    .map(|author| author.login)
                    .unwrap_or_default(),
                PrFieldName::CommitMessages => String::new(),
            };
            println!("{value}");
            0
        }
    }
}

fn fetch_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<GhPrFieldSnapshot, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("-R")
        .arg(repo_name)
        .arg("--json")
        .arg("state,baseRefName,headRefName,title,body,author")
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    common_json::from_json_str::<GhPrFieldSnapshot>(&json).map_err(|err| err.to_string())
}

fn fetch_commit_messages(pr_number: &str, repo_name: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("api")
        .arg(format!("repos/{repo_name}/pulls/{pr_number}/commits"))
        .arg("--paginate")
        .arg("--jq")
        .arg(".[].commit.message")
        .output()
        .map_err(|err| format!("Failed to execute gh api pulls/commits: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end_matches('\n')
        .to_string())
}
