use crate::pr::scan::scan_directives;

#[test]
fn scan_directives_extracts_event_decision_duplicate() {
    let text = "Closes #12\nPart of #8\nDirective decision: #12 => reopen\n#12 duplicate of #9";
    let records = scan_directives(text, false);
    assert_eq!(records.len(), 5);
}

#[test]
fn scan_directives_unique_deduplicates() {
    let text = "Closes #12\nCloses #12";
    let records = scan_directives(text, true);
    assert_eq!(records.len(), 1);
}
