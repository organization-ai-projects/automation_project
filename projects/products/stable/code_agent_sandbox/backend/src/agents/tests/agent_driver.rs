use crate::agents::agent_driver::parse_diff_touched_files;

#[test]
fn parse_diff_touched_files_extracts_unique_paths() {
    let diff = "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@\n--- a/src/main.rs\n+++ b/src/main.rs\n+++ b/src/lib.rs\n";
    let files = parse_diff_touched_files(diff);

    assert_eq!(
        files,
        vec!["src/lib.rs".to_string(), "src/main.rs".to_string()]
    );
}
