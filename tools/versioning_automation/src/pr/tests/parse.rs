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

#[test]
fn pr_directives_with_input_file_returns_zero() {
    let file_path = "/tmp/va_pr_directives_input.txt";
    std::fs::write(file_path, "Closes #12").expect("write input file");
    let args = vec![
        "directives".to_string(),
        "--input-file".to_string(),
        file_path.to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
    std::fs::remove_file(file_path).expect("remove input file");
}

#[test]
fn pr_directives_with_missing_input_file_returns_error() {
    let args = vec![
        "directives".to_string(),
        "--input-file".to_string(),
        "/tmp/va_pr_missing_input_file.txt".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_auto_add_closes_requires_pr_flag() {
    let args = vec!["auto-add-closes".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directive_conflicts_requires_input() {
    let args = vec!["directive-conflicts".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directive_conflicts_with_text_returns_zero() {
    let args = vec![
        "directive-conflicts".to_string(),
        "--text".to_string(),
        "Closes #1\nReopen #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_directives_state_requires_input() {
    let args = vec!["directives-state".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directives_state_with_text_returns_zero() {
    let args = vec![
        "directives-state".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_closure_refs_requires_input() {
    let args = vec!["closure-refs".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_closure_refs_with_text_returns_zero() {
    let args = vec![
        "closure-refs".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_issue_decision_requires_minimum_fields() {
    let args = vec!["issue-decision".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_decision_with_required_fields_returns_zero() {
    let args = vec![
        "issue-decision".to_string(),
        "--action".to_string(),
        "Closes".to_string(),
        "--issue".to_string(),
        "#42".to_string(),
        "--default-category".to_string(),
        "Mixed".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_closure_marker_requires_required_flags() {
    let args = vec!["closure-marker".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_closure_marker_with_required_flags_returns_zero() {
    let args = vec![
        "closure-marker".to_string(),
        "--text".to_string(),
        "Closes #42".to_string(),
        "--keyword-pattern".to_string(),
        "closes".to_string(),
        "--issue".to_string(),
        "#42".to_string(),
        "--mode".to_string(),
        "apply".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_resolve_category_requires_flags() {
    let args = vec!["resolve-category".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_resolve_category_with_required_flags_returns_zero() {
    let args = vec![
        "resolve-category".to_string(),
        "--label-category".to_string(),
        "Unknown".to_string(),
        "--title-category".to_string(),
        "UI".to_string(),
        "--default-category".to_string(),
        "Mixed".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}
