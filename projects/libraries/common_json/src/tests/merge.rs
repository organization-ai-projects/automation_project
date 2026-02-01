// projects/libraries/common_json/src/tests/merge.rs
use crate::{MergeStrategy, object};
use crate::merge::{merge, contains};

#[test]
fn test_merge() {
    let target = object();
    let source = object();
    let result = merge(&target, &source, MergeStrategy::Replace);
    assert!(result.is_object());
}

#[test]
fn test_contains() {
    let haystack = object();
    let needle = object();
    assert!(contains(&haystack, &needle));
}