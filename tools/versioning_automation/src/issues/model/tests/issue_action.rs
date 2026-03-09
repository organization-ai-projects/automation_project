use crate::issues::model::{CreateOptions, IssueAction};

#[test]
fn issue_action_create_variant_can_be_built() {
    let action = IssueAction::Create(CreateOptions {
        title: "t".to_string(),
        context: "c".to_string(),
        problem: "p".to_string(),
        acceptances: vec!["a".to_string()],
        parent: "none".to_string(),
        labels: vec![],
        repo: None,
        dry_run: true,
    });

    match action {
        IssueAction::Create(opts) => assert_eq!(opts.parent, "none"),
        _ => panic!("expected create variant"),
    }
}
