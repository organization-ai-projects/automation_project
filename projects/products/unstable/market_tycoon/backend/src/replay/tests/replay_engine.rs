use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;

#[test]
fn replay_produces_report() {
    let rf = ReplayFile::new(42, 10, vec![]);
    let report = ReplayEngine::replay(&rf).unwrap();
    assert!(!report.run_hash.is_empty());
}
