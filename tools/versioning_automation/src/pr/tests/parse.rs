use crate::pr::run;

#[test]
fn pr_help_returns_zero() {
    let args = vec!["help".to_string()];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_directives_requires_input() {
    let args = vec!["directives".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directives_with_text_returns_zero() {
    let args = vec![
        "directives".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}
