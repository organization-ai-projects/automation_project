use crate::app;

#[test]
fn app_run_without_command_returns_ok() {
    let args = vec!["princeps".to_string()];
    let result = app::run(args);
    assert!(result.is_ok());
}
