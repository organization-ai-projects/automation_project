//! tools/versioning_automation/src/issues/commands/tests/subissue_refs_options.rs
use crate::issues::commands::subissue_refs_options::SubissueRefsOptions;

#[test]
fn test_run_subissue_refs() {
    let options = SubissueRefsOptions {
        owner: "test_owner".to_string(),
        repo: "test_repo".to_string(),
        issue: "123".to_string(),
    };
    let result = options.run_subissue_refs();
    assert_eq!(result, 0);
}
