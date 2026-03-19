use crate::automation::commands::CheckMergeConflictsOptions;

#[test]
fn check_merge_conflicts_options_can_be_built() {
    let _opts = CheckMergeConflictsOptions {
        remote: "origin".to_string(),
        base_branch: "main".to_string(),
    };
}
