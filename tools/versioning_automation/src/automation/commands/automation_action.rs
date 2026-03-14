use crate::automation::commands::audit_issue_status_options::AuditIssueStatusOptions;
use crate::automation::commands::audit_security_options::AuditSecurityOptions;
use crate::automation::commands::build_accounts_ui_options::BuildAccountsUiOptions;
use crate::automation::commands::build_and_check_ui_bundles_options::BuildAndCheckUiBundlesOptions;
use crate::automation::commands::build_ui_bundles_options::BuildUiBundlesOptions;
use crate::automation::commands::changed_crates_options::ChangedCratesOptions;
use crate::automation::commands::check_dependencies_options::CheckDependenciesOptions;
use crate::automation::commands::check_merge_conflicts_options::CheckMergeConflictsOptions;
use crate::automation::commands::check_priority_issues_options::CheckPriorityIssuesOptions;
use crate::automation::commands::ci_watch_pr_options::CiWatchPrOptions;
use crate::automation::commands::clean_artifacts_options::CleanArtifactsOptions;
use crate::automation::commands::labels_sync_options::LabelsSyncOptions;
use crate::automation::commands::pre_add_review_options::PreAddReviewOptions;
use crate::automation::commands::pre_push_check_options::PrePushCheckOptions;
use crate::automation::commands::release_prepare_options::ReleasePrepareOptions;
use crate::automation::commands::sync_main_dev_ci_options::SyncMainDevCiOptions;
use crate::automation::commands::test_coverage_options::TestCoverageOptions;

#[derive(Debug)]
pub(crate) enum AutomationAction {
    Help,
    AuditIssueStatus(AuditIssueStatusOptions),
    AuditSecurity(AuditSecurityOptions),
    BuildAccountsUi(BuildAccountsUiOptions),
    BuildUiBundles(BuildUiBundlesOptions),
    BuildAndCheckUiBundles(BuildAndCheckUiBundlesOptions),
    PreAddReview(PreAddReviewOptions),
    PrePushCheck(PrePushCheckOptions),
    ReleasePrepare(ReleasePrepareOptions),
    TestCoverage(TestCoverageOptions),
    ChangedCrates(ChangedCratesOptions),
    CheckMergeConflicts(CheckMergeConflictsOptions),
    CheckDependencies(CheckDependenciesOptions),
    CleanArtifacts(CleanArtifactsOptions),
    CheckPriorityIssues(CheckPriorityIssuesOptions),
    LabelsSync(LabelsSyncOptions),
    CiWatchPr(CiWatchPrOptions),
    SyncMainDevCi(SyncMainDevCiOptions),
}
