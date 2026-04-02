#[test]
fn test_details_start_with_test() {
    let details = "test_details";
    assert!(
        details.starts_with("test"),
        "Details should start with 'test'"
    );
}
