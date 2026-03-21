use serde::{Deserialize, Serialize};

use crate::gh_cli::output_trim;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct IssueRemoteSnapshot {
    #[serde(default)]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) body: String,
    #[serde(default)]
    pub(crate) state: String,
    #[serde(default, deserialize_with = "deserialize_labels")]
    pub(crate) labels: Vec<String>,
}

pub(crate) fn load_issue_remote_snapshot(
    issue_number: &str,
    repo: Option<&str>,
) -> Result<IssueRemoteSnapshot, String> {
    let mut args = vec![
        "issue",
        "view",
        issue_number,
        "--json",
        "title,body,labels,state",
    ];
    if let Some(repo_name) = repo {
        args.extend(["-R", repo_name]);
    }

    let json = output_trim(&args)?;
    common_json::from_json_str::<IssueRemoteSnapshot>(&json).map_err(|err| err.to_string())
}

pub(crate) fn issue_labels_raw(snapshot: &IssueRemoteSnapshot) -> String {
    snapshot
        .labels
        .iter()
        .filter(|name| !name.trim().is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join("||")
}

fn deserialize_labels<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct IssueLabel {
        #[serde(default)]
        name: String,
    }

    let labels = Vec::<IssueLabel>::deserialize(deserializer)?;
    Ok(labels.into_iter().map(|label| label.name).collect())
}
