use crate::model::candidate::Candidate;
use crate::model::voter_block::VoterBlock;
use crate::poll::poll_model::PollModel;

#[test]
fn poll_model_determinism() {
    let candidates = vec![
        Candidate::new("a", "Alice", 70, 60, 80, 30),
        Candidate::new("b", "Bob", 55, 75, 65, 45),
    ];
    let blocks = vec![
        VoterBlock::new("urban", "Urban", 50),
        VoterBlock::new("rural", "Rural", 50),
    ];
    let report1 = PollModel::compute(1, &candidates, &blocks);
    let report2 = PollModel::compute(1, &candidates, &blocks);
    assert_eq!(report1.day, report2.day, "day must match");
    assert_eq!(report1.results, report2.results, "results must match");
    assert_eq!(
        report1.block_breakdown, report2.block_breakdown,
        "block breakdown must match"
    );
}
