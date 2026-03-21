#[test]
fn marker_jq_filter_escapes_backslashes_and_quotes() {
    let filter = crate::issue_comment_upsert::build_marker_jq_filter("a\\b\"c");
    assert!(filter.contains("a\\\\b\\\"c"));
}
