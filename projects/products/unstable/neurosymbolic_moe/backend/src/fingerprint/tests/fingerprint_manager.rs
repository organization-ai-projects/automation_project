use crate::fingerprint::fingerprint_manager::FingerprintManager;

#[test]
fn test_add_fingerprint() {
    let mut manager = FingerprintManager::new(3);
    manager.add_fingerprint("abc".to_string());
    assert!(manager.contains("abc"));
}

#[test]
fn test_capacity_limit() {
    let mut manager = FingerprintManager::new(2);
    manager.add_fingerprint("abc".to_string());
    manager.add_fingerprint("def".to_string());
    manager.add_fingerprint("ghi".to_string());
    assert!(!manager.contains("abc"));
    assert!(manager.contains("def"));
    assert!(manager.contains("ghi"));
}
