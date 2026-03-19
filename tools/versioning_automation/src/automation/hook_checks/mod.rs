//! tools/versioning_automation/src/automation/hook_checks/mod.rs
mod pre_commit;
mod pre_push;

pub(super) use pre_commit::run_pre_commit_check;
pub(super) use pre_push::run_pre_push_check;

#[cfg(test)]
mod tests;
