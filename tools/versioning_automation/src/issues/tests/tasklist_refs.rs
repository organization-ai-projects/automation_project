//! tools/versioning_automation/src/issues/tests/tasklist_refs.rs
use crate::issues::extract_tasklist_refs;

#[test]
fn extract_tasklist_refs_returns_unique_sorted_issue_refs() {
    let body = "\
- [ ] prep #12 and #2
- [x] done #12
- [X] done #50
plain line #999
";

    let refs = extract_tasklist_refs(body);
    assert_eq!(
        refs,
        vec!["#12".to_string(), "#2".to_string(), "#50".to_string()]
    );
}

#[test]
fn extract_tasklist_refs_ignores_non_tasklist_lines() {
    let body = "\
text #1
* [ ] #2
- [] #3
- [ ] no issue
";
    let refs = extract_tasklist_refs(body);
    assert!(refs.is_empty());
}
