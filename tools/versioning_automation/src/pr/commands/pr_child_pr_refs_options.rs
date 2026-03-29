//! tools/versioning_automation/src/pr/commands/pr_child_pr_refs_options.rs
use std::collections::BTreeSet;

use crate::{
    pr::child_pr_refs::{
        commit_headlines_from_messages, extract_refs_from_headlines, extract_refs_from_text,
        extract_timeline_refs, fetch_pr_comments, fetch_timeline_refs,
    },
    pr_remote_snapshot::PrRemoteSnapshot,
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrChildPrRefsOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}

impl PrChildPrRefsOptions {
    pub(crate) fn run_child_pr_refs(self) -> i32 {
        let Ok(repo_name) = resolve_repo_name(self.repo) else {
            return 0;
        };

        let pr_snapshot = PrRemoteSnapshot::load_pr_remote_snapshot(&self.pr_number, &repo_name)
            .unwrap_or_default();
        let commit_headlines = commit_headlines_from_messages(&pr_snapshot.commit_messages);
        let pr_body = pr_snapshot.body;
        let pr_comments = fetch_pr_comments(&self.pr_number, &repo_name).unwrap_or_default();
        let timeline_refs = fetch_timeline_refs(&self.pr_number, &repo_name).unwrap_or_default();

        let mut refs = BTreeSet::new();
        for issue_key in extract_refs_from_headlines(&commit_headlines) {
            refs.insert(issue_key);
        }
        for issue_key in extract_refs_from_text(&pr_body) {
            refs.insert(issue_key);
        }
        for issue_key in extract_refs_from_text(&pr_comments) {
            refs.insert(issue_key);
        }
        for issue_key in extract_timeline_refs(&timeline_refs) {
            refs.insert(issue_key);
        }

        let self_ref = format!("#{}", self.pr_number);
        refs.remove(&self_ref);

        for issue_key in refs {
            println!("{issue_key}");
        }
        0
    }
}
