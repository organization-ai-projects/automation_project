use crate::pr::conflicts::build_conflict_report;

#[test]
fn resolves_explicit_decision() {
    let text = "Closes #42\nReopen #42\nDirective Decision: #42 => close";
    let report = build_conflict_report(text, 1);
    assert_eq!(report.resolved.len(), 1);
    assert_eq!(report.unresolved.len(), 0);
    assert_eq!(report.resolved[0].issue, "#42");
    assert_eq!(report.resolved[0].decision, "close");
    assert_eq!(report.resolved[0].origin, "explicit");
}

#[test]
fn resolves_inferred_for_single_source_branch() {
    let text = "Closes #42\nReopen #42";
    let report = build_conflict_report(text, 1);
    assert_eq!(report.resolved.len(), 1);
    assert_eq!(report.unresolved.len(), 0);
    assert_eq!(report.resolved[0].issue, "#42");
    assert_eq!(report.resolved[0].decision, "reopen");
    assert_eq!(report.resolved[0].origin, "inferred from latest directive");
}

#[test]
fn blocks_inferred_for_multi_source_branch() {
    let text = "Closes #42\nReopen #42";
    let report = build_conflict_report(text, 2);
    assert_eq!(report.resolved.len(), 0);
    assert_eq!(report.unresolved.len(), 1);
    assert_eq!(report.unresolved[0].issue, "#42");
}

#[test]
fn cancel_closes_clears_close_reopen_conflict() {
    let text = "Closes #42\nCancel-Closes #42\nReopen #42";
    let report = build_conflict_report(text, 1);
    assert!(report.resolved.is_empty());
    assert!(report.unresolved.is_empty());
}
