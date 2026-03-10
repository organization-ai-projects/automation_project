use crate::issues::commands::CreateOptions;

#[test]
fn create_options_can_be_built() {
    let value = CreateOptions {
        title: "t".to_string(),
        context: "c".to_string(),
        problem: "p".to_string(),
        acceptances: vec!["a".to_string()],
        parent: "none".to_string(),
        labels: vec!["issue".to_string()],
        repo: None,
        dry_run: true,
    };
    assert!(value.dry_run);
}
