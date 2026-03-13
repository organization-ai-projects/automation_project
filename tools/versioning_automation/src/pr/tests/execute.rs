#[test]
fn run_returns_error_code_for_unknown_subcommand() {
    let code = crate::pr::execute::run(&["not-a-valid-command".to_string()]);
    assert_eq!(code, 2);
}

#[test]
fn run_help_returns_success() {
    let code = crate::pr::execute::run(&["help".to_string()]);
    assert_eq!(code, 0);
}
