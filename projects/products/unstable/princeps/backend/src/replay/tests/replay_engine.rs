use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;

#[test]
fn replay_engine_rebuilds_report_from_seed_and_days() {
    let replay = ReplayFile::new(3, 5);
    let report_result = ReplayEngine.replay(&replay);
    assert!(report_result.is_ok());
    if let Ok(report) = report_result {
        assert_eq!(report.run_summary.seed, 3);
        assert_eq!(report.run_summary.days, 5);
    }
}
