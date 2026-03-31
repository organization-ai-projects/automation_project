use crate::analysis::CodeAnalysis;
use crate::analysis::FindingKind;
use crate::analysis::ScopeResolver;
use crate::analysis::Severity;
use crate::analysis::SymbolExtractor;

#[test]
fn detect_unused_variable() {
    let source = "let x = 42;\nlet y = 10;\nprintln!(\"{}\", y);\n";
    let findings = CodeAnalysis::analyze(source);
    assert!(findings.iter().any(|f| f.kind == FindingKind::UnusedVariable
        && f.message.contains("x")));
}

#[test]
fn no_finding_when_all_variables_used() {
    let source = "let x = 42;\nprintln!(\"{}\", x);\n";
    let findings = CodeAnalysis::analyze(source);
    let unused: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UnusedVariable)
        .collect();
    assert!(unused.is_empty());
}

#[test]
fn detect_scope_violation() {
    let source = "fn main() {\n  let x = 1;\n}\n}\n";
    let findings = CodeAnalysis::analyze(source);
    assert!(findings.iter().any(|f| f.kind == FindingKind::ScopeViolation));
}

#[test]
fn balanced_braces_no_scope_violation() {
    let source = "fn main() {\n  let x = 1;\n}\n";
    let scope = ScopeResolver::resolve(source);
    assert!(scope.balanced);
}

#[test]
fn symbol_extractor_finds_definitions() {
    let source = "let count = 0;\nconst MAX = 100;\nfn helper() {}\n";
    let table = SymbolExtractor::extract(source);
    let names: Vec<&str> = table.definitions.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"count"));
    assert!(names.contains(&"MAX"));
    assert!(names.contains(&"helper"));
}

#[test]
fn severity_variants() {
    assert_ne!(Severity::Hint, Severity::Error);
    assert_eq!(Severity::Warning, Severity::Warning);
}
