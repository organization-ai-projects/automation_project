use crate::report::match_report::MatchReport;

#[test]
fn match_report_build_computes_hash() {
    let report = MatchReport::build("tiny".to_string(), 99, vec![]);
    assert_eq!(report.map_name, "tiny");
    assert_eq!(report.seed, 99);
    assert_eq!(report.run_hash.len(), 64);
}
