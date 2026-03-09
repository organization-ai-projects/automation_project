//! tools/versioning_automation/src/issues/model/issue_action.rs
use crate::issues::model::{CloseOptions, CreateOptions, IssueTarget, ReadOptions, UpdateOptions};

#[derive(Debug, Clone)]
pub(crate) enum IssueAction {
    Help,
    Create(CreateOptions),
    Read(ReadOptions),
    Update(UpdateOptions),
    Close(CloseOptions),
    Reopen(IssueTarget),
    Delete(IssueTarget),
}
