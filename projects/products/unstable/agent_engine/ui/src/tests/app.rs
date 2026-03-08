#[test]
fn sample_task_json_contains_required_fields() {
    let json = crate::app::sample_task_json();
    assert!(json.contains("\"id\""));
    assert!(json.contains("\"steps\""));
    assert!(json.contains("\"SetOutput\""));
}
