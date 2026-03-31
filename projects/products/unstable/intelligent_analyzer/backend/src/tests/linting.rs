use crate::analysis::FindingKind;
use crate::linting::LintEngine;

#[test]
fn detect_trailing_whitespace() {
    let source = "let x = 1;   \nlet y = 2;\n";
    let findings = LintEngine::lint(source);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::Custom("trailing_whitespace".to_string()))
    );
}

#[test]
fn detect_todo_comment() {
    let source = "// TODO: fix this\nlet x = 1;\n";
    let findings = LintEngine::lint(source);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::Custom("todo_comment".to_string()))
    );
}

#[test]
fn detect_line_too_long() {
    let long_line = format!("let x = \"{}\";", "a".repeat(200));
    let source = format!("{long_line}\n");
    let findings = LintEngine::lint(&source);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::Custom("line_too_long".to_string()))
    );
}

#[test]
fn no_lint_on_clean_code() {
    let source = "let x = 1;\nlet y = x + 2;\n";
    let findings = LintEngine::lint(source);
    let trail: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::Custom("trailing_whitespace".to_string()))
        .collect();
    assert!(trail.is_empty());
}

#[test]
fn detect_missing_doc_comment() {
    let source = "pub fn helper() {}\n";
    let findings = LintEngine::lint(source);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::Custom("missing_doc_comment".to_string()))
    );
}
