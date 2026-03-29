//! tools/versioning_automation/src/pr/main_pr_ref_snapshot.rs
use serde::Deserialize;

use crate::{gh_cli::output_trim, repo_name::resolve_repo_name_optional};

#[derive(Debug, Deserialize)]
pub(crate) struct MainPrRefSnapshot {
    #[serde(default, rename = "baseRefName")]
    pub(crate) base_ref_name: String,
    #[serde(default, rename = "headRefName")]
    pub(crate) head_ref_name: String,
}

impl MainPrRefSnapshot {
    pub(crate) fn fetch_pr_refs(pr_number: &str) -> Result<Self, String> {
        let mut args = vec![
            "view".to_string(),
            pr_number.to_string(),
            "--json".to_string(),
            "baseRefName,headRefName".to_string(),
        ];
        if let Some(repo) = resolve_repo_name_optional(None) {
            args.push("-R".to_string());
            args.push(repo);
        }

        let borrowed = args.iter().map(String::as_str).collect::<Vec<&str>>();
        let mut full_args = vec!["pr"];
        full_args.extend(borrowed);
        let json = output_trim(&full_args)?;
        Self::parse_pr_refs(&json)
    }

    fn parse_pr_refs(json: &str) -> Result<Self, String> {
        common_json::from_json_str::<Self>(json).map_err(|err| err.to_string())
    }
}
