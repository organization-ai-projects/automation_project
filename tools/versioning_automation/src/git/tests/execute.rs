#[test]
fn run_help_returns_zero() {
    let args = vec!["help".to_string()];
    let code = super::super::execute::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn run_unknown_subcommand_returns_two() {
    let args = vec!["unknown-subcommand".to_string()];
    let code = super::super::execute::run(&args);
    assert_eq!(code, 2);
}
