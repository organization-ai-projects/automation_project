use crate::snapshot::snapshot_hash::SnapshotHash;

#[test]
fn deterministic_hash() {
    let h1 = SnapshotHash::compute("hello world");
    let h2 = SnapshotHash::compute("hello world");
    assert_eq!(h1, h2);
    assert_eq!(h1.len(), 64);
}

#[test]
fn different_inputs_different_hashes() {
    let h1 = SnapshotHash::compute("hello");
    let h2 = SnapshotHash::compute("world");
    assert_ne!(h1, h2);
}
