//! tools/versioning_automation/src/pr/tests/scan.rs
use crate::pr::DirectiveRecord;

#[test]
fn scan_directives_extracts_event_decision_duplicate() {
    let text = "Closes #12\nPart of #8\nDirective decision: #12 => reopen\n#12 duplicate of #9";
    let records = DirectiveRecord::scan_directives(text, false);
    assert_eq!(records.len(), 5);
}

#[test]
fn scan_directives_unique_deduplicates() {
    let text = "Closes #12\nCloses #12";
    let records = DirectiveRecord::scan_directives(text, true);
    assert_eq!(records.len(), 1);
}

#[test]
fn scan_directives_extracts_rejected_closing_event() {
    let text = "Fixes rejected #42";
    let records = DirectiveRecord::scan_directives(text, false);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].first, "Closes rejected");
    assert_eq!(records[0].second, "#42");
}

#[test]
fn scan_directives_extracts_cancel_closes_event() {
    let text = "Cancel-Closes #42";
    let records = DirectiveRecord::scan_directives(text, false);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].first, "Cancel-Closes");
    assert_eq!(records[0].second, "#42");
}
