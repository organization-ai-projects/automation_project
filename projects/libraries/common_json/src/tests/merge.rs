// projects/libraries/common_json/src/tests/merge.rs
use super::test_helpers::assert_json_object;
use crate::merge::{contains, merge};
use crate::{MergeStrategy, object};

#[test]
fn test_merge() {
    let target = object();
    let source = object();
    let result = merge(&target, &source, MergeStrategy::Replace);
    assert_json_object(&result);
}

#[test]
fn test_contains() {
    let haystack = object();
    let needle = object();
    assert!(contains(&haystack, &needle));
}
