//! tools/versioning_automation/src/issues/commands/tests/upsert_marker_comment_options.rs
use crate::issues::commands::upsert_marker_comment_options::UpsertMarkerCommentOptions;

#[test]
fn test_run_upsert_marker_comment() {
    let options = UpsertMarkerCommentOptions {
        repo: "test_repo".to_string(),
        issue: "123".to_string(),
        marker: "test_marker".to_string(),
        body: "Test comment body".to_string(),
        announce: true,
    };
    let result = options.run_upsert_marker_comment();
    assert_eq!(result, 0);
}
