use crate::antivirus::scan_result::ScanResult;

#[test]
fn clean_result_is_clean() {
    let result = ScanResult::clean();
    assert!(result.is_clean);
    assert!(result.matched_signatures.is_empty());
}

#[test]
fn infected_result_has_signatures() {
    let result = ScanResult::infected(
        vec!["Virus.A".to_string(), "Trojan.B".to_string()],
        "high",
    );
    assert!(!result.is_clean);
    assert_eq!(result.matched_signatures.len(), 2);
    assert_eq!(result.severity, "high");
}
