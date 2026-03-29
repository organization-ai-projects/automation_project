//! tools/versioning_automation/src/issues/issue_sync_plan.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IssueSyncPlan {
    pub(crate) reopen_issue: bool,
    pub(crate) add_done_in_dev_label: bool,
    pub(crate) remove_done_in_dev_label: bool,
}

impl IssueSyncPlan {
    pub(crate) fn plan_done_in_dev_sync(issue_state: &str, has_done_in_dev_label: bool) -> Self {
        Self {
            reopen_issue: false,
            add_done_in_dev_label: issue_state == "OPEN" && !has_done_in_dev_label,
            remove_done_in_dev_label: false,
        }
    }

    pub(crate) fn plan_reopen_sync(issue_state: &str, has_done_in_dev_label: bool) -> Self {
        Self {
            reopen_issue: issue_state == "CLOSED",
            add_done_in_dev_label: false,
            remove_done_in_dev_label: has_done_in_dev_label,
        }
    }
}
