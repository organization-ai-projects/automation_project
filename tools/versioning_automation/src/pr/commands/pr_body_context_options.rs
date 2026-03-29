//! tools/versioning_automation/src/pr/commands/pr_body_context_options.rs
use crate::{pr_remote_snapshot::PrRemoteSnapshot, repo_name::resolve_repo_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrBodyContextOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}

impl PrBodyContextOptions {
    pub(crate) fn run_body_context(self) -> i32 {
        let Ok(repo_name) = resolve_repo_name(self.repo) else {
            return 0;
        };

        let Ok(snapshot) = PrRemoteSnapshot::load_pr_remote_snapshot(&self.pr_number, &repo_name)
        else {
            return 0;
        };

        let labels_raw = snapshot
            .labels
            .iter()
            .map(|label| label.name.clone())
            .filter(|name: &String| !name.trim().is_empty())
            .collect::<Vec<_>>()
            .join("||");
        println!("{}\x1f{}\x1f{}", snapshot.title, snapshot.body, labels_raw);
        0
    }
}
