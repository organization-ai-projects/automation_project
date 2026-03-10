use crate::pr::closure_marker::{apply_marker, remove_marker};

#[test]
fn apply_inserts_rejected_once() {
    let text = "Closes #42\nCloses rejected #42";
    let once = apply_marker(text, "closes", "#42").expect("apply marker");
    let twice = apply_marker(&once, "closes", "#42").expect("apply marker second pass");
    assert_eq!(once, twice);
}

#[test]
fn remove_deletes_rejected_marker() {
    let text = "Closes rejected #42";
    let out = remove_marker(text, "closes", "#42").expect("remove marker");
    assert_eq!(out, "Closes #42");
}
