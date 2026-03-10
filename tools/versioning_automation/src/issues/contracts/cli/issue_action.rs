//! tools/versioning_automation/src/issues/contracts/cli/issue_action.rs
use crate::issues::contracts::cli::{
    CloseOptions, CreateOptions, IssueTarget, ReadOptions, ReevaluateOptions, UpdateOptions,
};

#[derive(Debug, Clone)]
pub(crate) enum IssueAction {
    Help,
    Create(CreateOptions),
    Read(ReadOptions),
    Update(UpdateOptions),
    Close(CloseOptions),
    Reopen(IssueTarget),
    Delete(IssueTarget),
    Reevaluate(ReevaluateOptions),
}
