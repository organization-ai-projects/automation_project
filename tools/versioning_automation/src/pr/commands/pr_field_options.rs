//! tools/versioning_automation/src/pr/commands/pr_field_options.rs
use crate::{
    pr::commands::pr_field_name::PrFieldName, pr_remote_snapshot::PrRemoteSnapshot,
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrFieldOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
    pub(crate) name: PrFieldName,
}

impl PrFieldOptions {
    pub(crate) fn run_field(self) -> i32 {
        let Ok(repo_name) = resolve_repo_name(self.repo) else {
            println!();
            return 0;
        };

        match self.name {
            PrFieldName::CommitMessages => {
                let out = PrRemoteSnapshot::load_pr_remote_snapshot(&self.pr_number, &repo_name)
                    .map(|snapshot| snapshot.commit_messages)
                    .unwrap_or_default();
                print!("{out}");
                0
            }
            _ => {
                let snapshot =
                    PrRemoteSnapshot::load_pr_remote_snapshot(&self.pr_number, &repo_name)
                        .unwrap_or_default();
                let value = match self.name {
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
}
