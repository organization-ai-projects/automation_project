use crate::model::candidate_id::CandidateId;
use crate::poll::poll_report::PollReport;
use std::collections::BTreeMap;

#[test]
fn poll_report_leader_returns_top_candidate() {
    let mut results = BTreeMap::new();
    results.insert(CandidateId::new("a"), 0.45);
    results.insert(CandidateId::new("b"), 0.55);

    let report = PollReport {
        day: 10,
        results,
        block_breakdown: BTreeMap::new(),
    };

    let leader = report.leader();
    assert!(leader.is_some());
    if let Some(candidate_id) = leader {
        assert_eq!(candidate_id.to_string(), "b");
    }
}
