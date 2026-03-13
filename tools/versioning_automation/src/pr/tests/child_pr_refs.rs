use crate::pr::run;

#[test]
fn child_pr_refs_rejects_non_numeric_pr() {
    let args = vec![
        "child-pr-refs".to_string(),
        "--pr".to_string(),
        "abc".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn child_pr_refs_rejects_unknown_option() {
    let args = vec![
        "child-pr-refs".to_string(),
        "--pr".to_string(),
        "42".to_string(),
        "--unknown".to_string(),
        "x".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}
