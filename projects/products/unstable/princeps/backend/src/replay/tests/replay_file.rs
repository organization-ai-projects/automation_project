use crate::replay::replay_file::ReplayFile;

#[test]
fn replay_file_new_is_empty() {
    let replay = ReplayFile::new(8, 12);
    assert_eq!(replay.seed, 8);
    assert_eq!(replay.days, 12);
    assert!(replay.actions.is_empty());
    assert!(replay.drawn_event_indices.is_empty());
}
