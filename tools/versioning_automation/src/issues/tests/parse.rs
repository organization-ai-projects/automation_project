use crate::issues;

#[test]
fn parse_via_public_run_help_works() {
    let args = vec!["help".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}
