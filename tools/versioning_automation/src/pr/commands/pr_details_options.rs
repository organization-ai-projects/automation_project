//! tools/versioning_automation/src/pr/commands/pr_details_options.rs
use serde::Serialize;

use crate::{pr_remote_snapshot::PrRemoteSnapshot, repo_name::resolve_repo_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDetailsOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}

impl PrDetailsOptions {
    pub(crate) fn run_details(self) -> i32 {
        let Ok(repo_name) = resolve_repo_name(self.repo) else {
            println!();
            return 0;
        };

        let pr_snapshot = PrRemoteSnapshot::load_pr_remote_snapshot(&self.pr_number, &repo_name)
            .unwrap_or_default();
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
            commit_messages: pr_snapshot.commit_messages,
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
}
