use crate::replay::replay_file::ReplayFile;

#[test]
fn new_replay_file() {
    let rf = ReplayFile::new(42, 10, vec![]);
    assert_eq!(rf.seed, 42);
    assert_eq!(rf.ticks, 10);
    assert!(rf.events.is_empty());
}
