use crate::pr::commands::pr_body_context_options::PrBodyContextOptions;
use crate::pr::contracts::github::issue_label::IssueLabel;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::repo_name::resolve_repo_name;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BodyContext {
    #[serde(default)]
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    labels: Vec<IssueLabel>,
}

pub(crate) fn run_body_context(opts: PrBodyContextOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };

    let Ok(snapshot) = fetch_pr_snapshot(&opts.pr_number, &repo_name) else {
        return 0;
    };

    let labels_raw = snapshot
        .labels
        .iter()
        .map(|label| label.name.clone())
        .filter(|name| !name.trim().is_empty())
        .collect::<Vec<_>>()
        .join("||");
    println!("{}\x1f{}\x1f{}", snapshot.title, snapshot.body, labels_raw);
    0
}

fn fetch_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<BodyContext, String> {
    let json = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "title,body,labels",
        ],
    )?;
    common_json::from_json_str::<BodyContext>(&json).map_err(|err| err.to_string())
}
