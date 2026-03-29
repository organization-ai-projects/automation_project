//! Test unitaire pour NeutralizeRefBuckets

#[cfg(test)]
use crate::issues::neutralize_ref_buckets::NeutralizeRefBuckets;

#[test]
fn test_collect_neutralize_refs() {
    let text = "Closes #123 Fixes rejected #456";
    let buckets = NeutralizeRefBuckets::collect_neutralize_refs(text);

    assert_eq!(buckets.0.len(), 1);
    assert_eq!(buckets.0[0].1, "#123");
    assert_eq!(buckets.1.len(), 1);
    assert_eq!(buckets.1[0].1, "#456");
}

#[test]
fn test_collect_neutralize_refs_empty() {
    let text = "";
    let buckets = NeutralizeRefBuckets::collect_neutralize_refs(text);

    assert!(buckets.0.is_empty());
    assert!(buckets.1.is_empty());
}
