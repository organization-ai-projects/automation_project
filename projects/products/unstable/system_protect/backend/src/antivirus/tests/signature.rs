use crate::antivirus::signature::Signature;

#[test]
fn signature_matches_pattern_in_payload() {
    let sig = Signature::new("test-sig", "malicious_code", "high");
    assert!(sig.matches("file contains malicious_code here"));
}

#[test]
fn signature_does_not_match_absent_pattern() {
    let sig = Signature::new("test-sig", "malicious_code", "high");
    assert!(!sig.matches("clean file content"));
}
