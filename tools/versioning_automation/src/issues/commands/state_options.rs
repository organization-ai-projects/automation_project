//! tools/versioning_automation/src/issues/commands/state_options.rs

use crate::issues::execute::gh_issue_state_or_empty;
#[derive(Debug, Clone)]
pub(crate) struct StateOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl StateOptions {
    pub(crate) fn run_state(self) -> i32 {
        let state = gh_issue_state_or_empty(self.repo.as_deref(), &self.issue);
        if !state.is_empty() {
            println!("{state}");
        }
        0
    }
}
