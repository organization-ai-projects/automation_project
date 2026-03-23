use crate::pr::breaking_detect::{labels_indicate_breaking, text_indicates_breaking};

#[test]
fn labels_detect_breaking_label() {
    assert!(labels_indicate_breaking("foo||breaking||bar"));
}

#[test]
fn text_detect_breaking_checked_box() {
    assert!(text_indicates_breaking("- [x] Breaking change"));
}

#[test]
fn text_ignores_non_breaking_sentence() {
    assert!(!text_indicates_breaking("non-breaking change"));
}
