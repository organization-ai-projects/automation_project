use crate::app::action::Action;

#[test]
fn action_load_log_file_has_path() {
    let action = Action::LoadLogFile {
        path: "test.json".to_string(),
    };
    match action {
        Action::LoadLogFile { path } => assert_eq!(path, "test.json"),
        _ => panic!("Expected LoadLogFile"),
    }
}

#[test]
fn action_clone_is_equal() {
    let action = Action::ClearPanelData;
    let cloned = action.clone();
    assert_eq!(action, cloned);
}

#[test]
fn action_debug_format() {
    let action = Action::ExportSnapshot;
    let debug = format!("{action:?}");
    assert!(debug.contains("ExportSnapshot"));
}
