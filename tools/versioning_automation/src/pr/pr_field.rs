use serde::Deserialize;

use crate::pr::commands::pr_field_name::PrFieldName;
use crate::pr::commands::pr_field_options::PrFieldOptions;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::repo_name::resolve_repo_name;

#[derive(Debug, Deserialize)]
struct PrField {
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
            let snapshot = fetch_pr_snapshot(&opts.pr_number, &repo_name).unwrap_or(PrField {
                state: String::new(),
                base_ref_name: String::new(),
                head_ref_name: String::new(),
                title: String::new(),
                body: String::new(),
                author_login: String::new(),
            });
            let value = match opts.name {
                PrFieldName::State => snapshot.state,
                PrFieldName::BaseRefName => snapshot.base_ref_name,
                PrFieldName::HeadRefName => snapshot.head_ref_name,
                PrFieldName::Title => snapshot.title,
                PrFieldName::Body => snapshot.body,
                PrFieldName::AuthorLogin => snapshot.author_login,
                PrFieldName::CommitMessages => String::new(),
            };
            println!("{value}");
            0
        }
    }
}

fn fetch_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<PrField, String> {
    let json = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "state,baseRefName,headRefName,title,body,author",
        ],
    )?;
    common_json::from_json_str::<PrField>(&json).map_err(|err| err.to_string())
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
