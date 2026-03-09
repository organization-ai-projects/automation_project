use crate::issues;

#[test]
fn render_usage_path_returns_zero() {
    let args = vec!["--help".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}
