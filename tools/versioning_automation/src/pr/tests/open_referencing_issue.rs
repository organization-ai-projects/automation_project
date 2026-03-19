use crate::pr::run;

#[test]
fn open_referencing_issue_rejects_non_numeric_issue() {
    let args = vec![
        "open-referencing-issue".to_string(),
        "--issue".to_string(),
        "abc".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn open_referencing_issue_rejects_unknown_option() {
    let args = vec![
        "open-referencing-issue".to_string(),
        "--issue".to_string(),
        "42".to_string(),
        "--unknown".to_string(),
        "x".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}
