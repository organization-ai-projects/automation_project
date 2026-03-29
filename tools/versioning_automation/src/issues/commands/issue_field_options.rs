//! tools/versioning_automation/src/issues/commands/issue_field_options.rs
use crate::{issue_remote_snapshot::IssueRemoteSnapshot, issues::commands::IssueFieldName};

#[derive(Debug, Clone)]
pub(crate) struct IssueFieldOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
    pub(crate) name: IssueFieldName,
}

impl IssueFieldOptions {
    pub(crate) fn run_field(self) -> i32 {
        let Ok(snapshot) =
            IssueRemoteSnapshot::load_issue_remote_snapshot(&self.issue, self.repo.as_deref())
        else {
            println!();
            return 0;
        };

        match self.name {
            IssueFieldName::Title => println!("{}", snapshot.title),
            IssueFieldName::Body => println!("{}", snapshot.body),
            IssueFieldName::LabelsRaw => {
                println!("{}", IssueRemoteSnapshot::issue_labels_raw(&snapshot))
            }
        }

        0
    }
}
