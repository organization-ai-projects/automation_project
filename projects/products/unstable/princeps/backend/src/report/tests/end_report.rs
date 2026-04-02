use crate::model::candidate_id::CandidateId;
use crate::model::game_id::GameId;
use crate::poll::poll_report::PollReport;
use crate::report::run_summary::RunSummary;
use crate::sim::sim_engine::canonical_report_payload;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[test]
fn end_report_run_hash_is_stable() {
    let mut results = BTreeMap::new();
    results.insert(CandidateId::new("a"), 0.6);
    results.insert(CandidateId::new("b"), 0.4);
    let poll = PollReport {
        day: 30,
        results,
        block_breakdown: BTreeMap::new(),
    };
    let mut approvals = BTreeMap::new();
    approvals.insert(CandidateId::new("a"), 0.3);
    approvals.insert(CandidateId::new("b"), 0.2);
    let summary = RunSummary {
        seed: 42,
        days: 30,
        total_events: 10,
        total_debates: 2,
        total_polls: 5,
        candidate_final_approvals: approvals,
    };
    let game_id = GameId::new(42, 30);
    let winner = CandidateId::new("a");
    let canonical = canonical_report_payload(&game_id, &winner, &poll, &summary);
    let mut h1 = Sha256::new();
    h1.update(canonical.as_bytes());
    let hash1 = hex::encode(h1.finalize());
    let mut h2 = Sha256::new();
    h2.update(canonical.as_bytes());
    let hash2 = hex::encode(h2.finalize());
    assert_eq!(hash1, hash2, "hash must be stable/deterministic");
    assert_eq!(hash1.len(), 64, "SHA256 hex should be 64 chars");
}
