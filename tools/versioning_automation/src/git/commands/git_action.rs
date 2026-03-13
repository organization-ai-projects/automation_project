use crate::git::commands::add_commit_push_options::AddCommitPushOptions;
use crate::git::commands::clean_branches_options::CleanBranchesOptions;
use crate::git::commands::clean_local_gone_options::CleanLocalGoneOptions;
use crate::git::commands::cleanup_after_pr_options::CleanupAfterPrOptions;
use crate::git::commands::create_after_delete_options::CreateAfterDeleteOptions;
use crate::git::commands::create_branch_options::CreateBranchOptions;
use crate::git::commands::create_work_branch_options::CreateWorkBranchOptions;
use crate::git::commands::delete_branch_options::DeleteBranchOptions;
use crate::git::commands::finish_branch_options::FinishBranchOptions;
use crate::git::commands::push_branch_options::PushBranchOptions;

#[derive(Debug)]
pub(crate) enum GitAction {
    Help,
    CreateBranch(CreateBranchOptions),
    CreateWorkBranch(CreateWorkBranchOptions),
    PushBranch(PushBranchOptions),
    AddCommitPush(AddCommitPushOptions),
    DeleteBranch(DeleteBranchOptions),
    FinishBranch(FinishBranchOptions),
    CreateAfterDelete(CreateAfterDeleteOptions),
    CleanLocalGone(CleanLocalGoneOptions),
    CleanBranches(CleanBranchesOptions),
    CleanupAfterPr(CleanupAfterPrOptions),
}
