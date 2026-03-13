use crate::automation::commands::check_priority_issues_options::CheckPriorityIssuesOptions;
use crate::automation::commands::ci_watch_pr_options::CiWatchPrOptions;
use crate::automation::commands::labels_sync_options::LabelsSyncOptions;
use crate::automation::commands::sync_main_dev_ci_options::SyncMainDevCiOptions;

#[derive(Debug)]
pub(crate) enum AutomationAction {
    Help,
    CheckPriorityIssues(CheckPriorityIssuesOptions),
    LabelsSync(LabelsSyncOptions),
    CiWatchPr(CiWatchPrOptions),
    SyncMainDevCi(SyncMainDevCiOptions),
}
