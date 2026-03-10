use crate::actions::Action;

#[test]
fn estimated_file_touch_count_matches_action_kind() {
    assert_eq!(
        Action::ReadFile { path: "a".into() }.estimated_file_touch_count(),
        1
    );
    assert_eq!(
        Action::RunCargo {
            subcommand: "check".into(),
            args: vec![],
        }
        .estimated_file_touch_count(),
        0
    );
}
