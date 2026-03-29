//! tools/versioning_automation/src/pr_remote_snapshot.rs
use serde::Deserialize;

use crate::gh_cli::output_trim;
use crate::pr::IssueLabel;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub(crate) struct PrRemoteSnapshot {
    #[serde(default)]
    pub(crate) number: u64,
    #[serde(default)]
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) state: String,
    #[serde(default, rename = "baseRefName")]
    pub(crate) base_ref_name: String,
    #[serde(default, rename = "headRefName")]
    pub(crate) head_ref_name: String,
    #[serde(default)]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) body: String,
    #[serde(default)]
    pub(crate) labels: Vec<IssueLabel>,
    #[serde(default, deserialize_with = "deserialize_author_login")]
    pub(crate) author_login: String,
    #[serde(skip)]
    pub(crate) commit_messages: String,
}

impl PrRemoteSnapshot {
    pub(crate) fn load_pr_remote_snapshot(
        pr_number: &str,
        repo_name: &str,
    ) -> Result<Self, String> {
        let snapshot_json = output_trim(&[
            "pr",
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "number,url,state,baseRefName,headRefName,title,body,labels,author",
        ])?;
        let mut snapshot = Self::parse_pr_remote_snapshot(&snapshot_json)?;
        snapshot.commit_messages = fetch_pr_commit_messages(pr_number, repo_name)?;
        Ok(snapshot)
    }

    pub(crate) fn pr_text_payload_from_snapshot(snapshot: &PrRemoteSnapshot) -> String {
        format!(
            "{}\n{}\n{}",
            snapshot.title, snapshot.body, snapshot.commit_messages
        )
    }

    fn parse_pr_remote_snapshot(json: &str) -> Result<Self, String> {
        common_json::from_json_str::<Self>(json).map_err(|err| err.to_string())
    }
}

fn fetch_pr_commit_messages(pr_number: &str, repo_name: &str) -> Result<String, String> {
    output_trim(&[
        "api",
        &format!("repos/{repo_name}/pulls/{pr_number}/commits"),
        "--paginate",
        "--jq",
        ".[].commit.message",
    ])
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
