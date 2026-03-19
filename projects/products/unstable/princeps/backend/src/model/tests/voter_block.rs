use crate::model::voter_block::VoterBlock;

#[test]
fn voter_block_new_initializes_empty_maps() {
    let block = VoterBlock::new("urban", "Urban", 120);
    assert_eq!(block.id, "urban");
    assert_eq!(block.name, "Urban");
    assert_eq!(block.size, 120);
    assert!(block.preferences.is_empty());
    assert!(block.sensitivities.is_empty());
    assert!(block.support.is_empty());
}
