use serde::{Deserialize, Serialize};

use crate::pr::commands::pr_details_options::PrDetailsOptions;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::repo_name::resolve_repo_name;

#[derive(Debug, Deserialize)]
struct Details {
    #[serde(default)]
    number: u64,
    #[serde(default)]
    url: String,
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
    #[serde(default, deserialize_with = "deserialize_author_login")]
    author_login: String,
}

fn deserialize_author_login<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct AuthorLogin {
        #[serde(default)]
        login: String,
    }
    let value = Option::<AuthorLogin>::deserialize(deserializer)?;
    Ok(value.map(|entry| entry.login).unwrap_or_default())
}

pub(crate) fn run_details(opts: PrDetailsOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    let pr_snapshot = fetch_pr_snapshot(&opts.pr_number, &repo_name).unwrap_or(Details {
        number: 0,
        url: String::new(),
        state: String::new(),
        base_ref_name: String::new(),
        head_ref_name: String::new(),
        title: String::new(),
        body: String::new(),
        author_login: String::new(),
    });
    let commit_messages = fetch_commit_messages(&opts.pr_number, &repo_name).unwrap_or_default();
    #[derive(Debug, Serialize)]
    struct DetailsOutput {
        number: u64,
        url: String,
        state: String,
        base_ref_name: String,
        head_ref_name: String,
        author_login: String,
        title: String,
        body: String,
        commit_messages: String,
    }
    let output = DetailsOutput {
        number: pr_snapshot.number,
        url: pr_snapshot.url,
        state: pr_snapshot.state,
        base_ref_name: pr_snapshot.base_ref_name,
        head_ref_name: pr_snapshot.head_ref_name,
        author_login: pr_snapshot.author_login,
        title: pr_snapshot.title,
        body: pr_snapshot.body,
        commit_messages,
    };

    match common_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(err) => {
            eprintln!("failed to serialize pr details: {err}");
            1
        }
    }
}

fn fetch_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<Details, String> {
    let json = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "number,url,state,baseRefName,headRefName,title,body,author",
        ],
    )?;
    common_json::from_json_str::<Details>(&json).map_err(|err| err.to_string())
}

fn fetch_commit_messages(pr_number: &str, repo_name: &str) -> Result<String, String> {
    gh_output_trim_end_newline(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{pr_number}/commits"),
            "--paginate",
            "--jq",
            ".[].commit.message",
        ],
    )
}
