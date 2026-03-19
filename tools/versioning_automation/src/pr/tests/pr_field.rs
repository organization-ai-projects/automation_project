use crate::pr::run;

#[test]
fn pr_field_rejects_non_numeric_pr() {
    let args = vec![
        "field".to_string(),
        "--pr".to_string(),
        "abc".to_string(),
        "--name".to_string(),
        "state".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_field_rejects_zero_pr() {
    let args = vec![
        "field".to_string(),
        "--pr".to_string(),
        "0".to_string(),
        "--name".to_string(),
        "state".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_field_rejects_unknown_field_name() {
    let args = vec![
        "field".to_string(),
        "--pr".to_string(),
        "42".to_string(),
        "--name".to_string(),
        "unknown-field".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}
